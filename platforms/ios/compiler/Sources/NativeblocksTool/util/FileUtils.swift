import Foundation

extension FileManager {
    func createDirectory(atPath path: String) throws {
        let url = URL(fileURLWithPath: path)
        try createDirectory(at: url, withIntermediateDirectories: true, attributes: nil)
    }

    func deleteDirectory(atPath path: String) throws {
        let url = URL(fileURLWithPath: path)
        try removeItem(at: url)
    }

    func getFiles(from directory: URL) throws -> [URL] {
        var filePaths: [URL] = []
        let contents = try contentsOfDirectory(
            at: directory, includingPropertiesForKeys: nil, options: .skipsHiddenFiles)
        for url in contents {
            var isDirectory: ObjCBool = false
            if fileExists(atPath: url.path, isDirectory: &isDirectory), isDirectory.boolValue {
                try filePaths.append(contentsOf: getFiles(from: url))
            } else if url.pathExtension == "swift" {
                print(url.absoluteString)
                filePaths.append(url)
            }
        }
        return filePaths
    }

    func getFilesContent(from directory: URL) throws -> [String] {
        let files = try getFiles(from: directory)
        return files.compactMap { filePath in
            try? String(contentsOf: filePath, encoding: .utf8)
        }
    }

    static func getFilesContent(from files: [URL]) throws -> [String] {
        return files.compactMap { filePath in
            try? String(contentsOf: filePath, encoding: .utf8)
        }
    }
}

extension Data {
    public func toString() -> String? {
        return String(data: self, encoding: .utf8)
    }
}
