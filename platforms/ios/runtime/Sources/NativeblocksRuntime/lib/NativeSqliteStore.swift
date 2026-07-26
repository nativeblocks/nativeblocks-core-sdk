import Foundation
import NativeblocksRuntimeFFI
import SQLite3

private let SQLITE_TRANSIENT = unsafeBitCast(-1, to: sqlite3_destructor_type.self)

internal final class NativeSqliteHelper {

    private static let table = "nativeblocks_cache"
    private static let columnKey = "cache_key"
    private static let columnValue = "cache_value"
    private static let columnExpiry = "cache_expiry"

    private let lock = NSLock()
    private var handle: OpaquePointer?

    init(databaseName: String) throws {
        let directory = try FileManager.default.url(
            for: .applicationSupportDirectory,
            in: .userDomainMask,
            appropriateFor: nil,
            create: true
        )
        try FileManager.default.createDirectory(at: directory, withIntermediateDirectories: true)
        let path = directory.appendingPathComponent("\(databaseName).sqlite").path

        var db: OpaquePointer?
        guard sqlite3_open(path, &db) == SQLITE_OK, let opened = db else {
            let reason = db.map { String(cString: sqlite3_errmsg($0)) } ?? "unknown error"
            sqlite3_close(db)
            throw NbError.Failure(reason: "Cannot open \(path): \(reason)", errorType: .cache, errorCode: nil)
        }
        self.handle = opened

        try execute(
            """
            CREATE TABLE IF NOT EXISTS \(Self.table) (
                \(Self.columnKey) TEXT PRIMARY KEY,
                \(Self.columnValue) BLOB NOT NULL,
                \(Self.columnExpiry) INTEGER
            )
            """
        )
    }


    func save(key: String, value: Data, ttlMillis: Int64?) throws {
        lock.lock()
        defer { lock.unlock() }

        let sql = """
            INSERT OR REPLACE INTO \(Self.table)
            (\(Self.columnKey), \(Self.columnValue), \(Self.columnExpiry)) VALUES (?, ?, ?)
            """
        let statement = try prepare(sql)
        defer { sqlite3_finalize(statement) }

        sqlite3_bind_text(statement, 1, key, -1, SQLITE_TRANSIENT)
        _ = value.withUnsafeBytes { buffer in
            sqlite3_bind_blob(statement, 2, buffer.baseAddress, Int32(buffer.count), SQLITE_TRANSIENT)
        }
        if let ttlMillis {
            sqlite3_bind_int64(statement, 3, Self.nowMillis() + ttlMillis)
        } else {
            sqlite3_bind_null(statement, 3)
        }

        guard sqlite3_step(statement) == SQLITE_DONE else { throw failure() }
    }

    func load(key: String) throws -> Data? {
        lock.lock()

        let sql = """
            SELECT \(Self.columnValue), \(Self.columnExpiry) FROM \(Self.table)
            WHERE \(Self.columnKey) = ?
            """
        let statement: OpaquePointer?
        do {
            statement = try prepare(sql)
        } catch {
            lock.unlock()
            throw error
        }
        sqlite3_bind_text(statement, 1, key, -1, SQLITE_TRANSIENT)

        guard sqlite3_step(statement) == SQLITE_ROW else {
            sqlite3_finalize(statement)
            lock.unlock()
            return nil
        }

        if sqlite3_column_type(statement, 1) != SQLITE_NULL {
            let expiry = sqlite3_column_int64(statement, 1)
            if Self.nowMillis() > expiry {
                sqlite3_finalize(statement)
                lock.unlock()
                // Evicting takes the lock again, hence the unlock above.
                try remove(key: key)
                return nil
            }
        }

        defer {
            sqlite3_finalize(statement)
            lock.unlock()
        }
        guard let bytes = sqlite3_column_blob(statement, 0) else { return Data() }
        return Data(bytes: bytes, count: Int(sqlite3_column_bytes(statement, 0)))
    }

    func remove(key: String) throws {
        lock.lock()
        defer { lock.unlock() }

        let statement = try prepare("DELETE FROM \(Self.table) WHERE \(Self.columnKey) = ?")
        defer { sqlite3_finalize(statement) }

        sqlite3_bind_text(statement, 1, key, -1, SQLITE_TRANSIENT)
        guard sqlite3_step(statement) == SQLITE_DONE else { throw failure() }
    }

    func clear() throws {
        lock.lock()
        defer { lock.unlock() }
        try execute("DELETE FROM \(Self.table)")
    }

    func close() {
        lock.lock()
        defer { lock.unlock() }
        guard let handle else { return }
        sqlite3_close(handle)
        self.handle = nil
    }

    deinit {
        if let handle { sqlite3_close(handle) }
    }

    private func prepare(_ sql: String) throws -> OpaquePointer? {
        guard let handle else {
            throw NbError.Failure(reason: "The cache database is closed", errorType: .cache, errorCode: nil)
        }
        var statement: OpaquePointer?
        guard sqlite3_prepare_v2(handle, sql, -1, &statement, nil) == SQLITE_OK else {
            sqlite3_finalize(statement)
            throw failure()
        }
        return statement
    }

    private func execute(_ sql: String) throws {
        let statement = try prepare(sql)
        defer { sqlite3_finalize(statement) }
        guard sqlite3_step(statement) == SQLITE_DONE else { throw failure() }
    }

    private func failure() -> NbError {
        let reason = handle.map { String(cString: sqlite3_errmsg($0)) } ?? "the cache database is closed"
        return NbError.Failure(reason: reason, errorType: .cache, errorCode: nil)
    }

    private static func nowMillis() -> Int64 {
        return Int64(Date().timeIntervalSince1970 * 1000)
    }
}

internal final class SqliteCacheProvider: CacheProvider, @unchecked Sendable {

    private let helper: NativeSqliteHelper

    init(helper: NativeSqliteHelper) {
        self.helper = helper
    }

    func saveBytes(key: String, value: Data, ttlMillis: Int64?) throws {
        try uniffiCallbackGuard(.cache) {
            try helper.save(key: key, value: value, ttlMillis: ttlMillis)
        }
    }

    func getBytes(key: String) throws -> Data? {
        try uniffiCallbackGuard(.cache) {
            try helper.load(key: key)
        }
    }

    func remove(key: String) throws {
        try uniffiCallbackGuard(.cache) {
            try helper.remove(key: key)
        }
    }

    func clear() throws {
        try uniffiCallbackGuard(.cache) {
            try helper.clear()
        }
    }

    func has(key: String) throws -> Bool {
        return try getBytes(key: key) != nil
    }

    func dispose() throws {
        try uniffiCallbackGuard(.cache) {
            helper.close()
        }
    }
}
