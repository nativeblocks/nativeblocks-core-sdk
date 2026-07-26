package io.nativeblocks.runtime.lib

import android.content.ContentValues
import android.content.Context
import android.database.sqlite.SQLiteDatabase
import android.database.sqlite.SQLiteOpenHelper
import io.nativeblocks.runtime.engine.CacheProvider
import io.nativeblocks.runtime.engine.ErrorType

internal class NativeSqliteHelper(
    context: Context,
    databaseName: String,
) : SQLiteOpenHelper(context.applicationContext, databaseName, null, DATABASE_VERSION) {

    override fun onCreate(db: SQLiteDatabase) {
        db.execSQL(
            "CREATE TABLE IF NOT EXISTS $TABLE (" +
                    "$COLUMN_KEY TEXT PRIMARY KEY, " +
                    "$COLUMN_VALUE BLOB NOT NULL, " +
                    "$COLUMN_EXPIRY INTEGER" +
                    ")"
        )
    }

    override fun onUpgrade(db: SQLiteDatabase, oldVersion: Int, newVersion: Int) {
        db.execSQL("DROP TABLE IF EXISTS $TABLE")
        onCreate(db)
    }

    companion object {
        private const val DATABASE_VERSION = 2
        const val TABLE = "nativeblocks_cache"
        const val COLUMN_KEY = "cache_key"
        const val COLUMN_VALUE = "cache_value"
        const val COLUMN_EXPIRY = "cache_expiry"
    }
}

internal class SqliteCacheProvider(
    private val helper: NativeSqliteHelper,
) : CacheProvider {

    override fun saveBytes(key: String, value: ByteArray, ttlMillis: Long?) = uniffiCallbackGuard(ErrorType.CACHE) {
        val values = ContentValues().apply {
            put(NativeSqliteHelper.COLUMN_KEY, key)
            put(NativeSqliteHelper.COLUMN_VALUE, value)
            if (ttlMillis != null) {
                put(NativeSqliteHelper.COLUMN_EXPIRY, System.currentTimeMillis() + ttlMillis)
            } else {
                putNull(NativeSqliteHelper.COLUMN_EXPIRY)
            }
        }
        helper.writableDatabase.insertWithOnConflict(
            NativeSqliteHelper.TABLE,
            null,
            values,
            SQLiteDatabase.CONFLICT_REPLACE,
        )
        Unit
    }

    override fun getBytes(key: String): ByteArray? = uniffiCallbackGuard(ErrorType.CACHE) {
        helper.readableDatabase.query(
            NativeSqliteHelper.TABLE,
            arrayOf(NativeSqliteHelper.COLUMN_VALUE, NativeSqliteHelper.COLUMN_EXPIRY),
            "${NativeSqliteHelper.COLUMN_KEY} = ?",
            arrayOf(key),
            null,
            null,
            null,
        ).use { cursor ->
            if (!cursor.moveToFirst()) return@uniffiCallbackGuard null
            val expiryIndex = cursor.getColumnIndexOrThrow(NativeSqliteHelper.COLUMN_EXPIRY)
            if (!cursor.isNull(expiryIndex)) {
                val expiry = cursor.getLong(expiryIndex)
                if (System.currentTimeMillis() > expiry) {
                    remove(key)
                    return@uniffiCallbackGuard null
                }
            }
            cursor.getBlob(cursor.getColumnIndexOrThrow(NativeSqliteHelper.COLUMN_VALUE))
        }
    }

    override fun remove(key: String) = uniffiCallbackGuard(ErrorType.CACHE) {
        helper.writableDatabase.delete(
            NativeSqliteHelper.TABLE,
            "${NativeSqliteHelper.COLUMN_KEY} = ?",
            arrayOf(key),
        )
        Unit
    }

    override fun clear() = uniffiCallbackGuard(ErrorType.CACHE) {
        helper.writableDatabase.delete(NativeSqliteHelper.TABLE, null, null)
        Unit
    }

    override fun has(key: String): Boolean = getBytes(key) != null

    override fun dispose() = uniffiCallbackGuard(ErrorType.CACHE) {
        helper.close()
    }

}
