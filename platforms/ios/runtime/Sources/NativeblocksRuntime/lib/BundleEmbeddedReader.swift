import Foundation
import NativeblocksRuntimeFFI

internal final class BundleEmbeddedReader: EmbeddedReader, @unchecked Sendable {

    private static let embeddedDir = "nativeblocksEmbedded"

    private let root: URL?

    init(bundle: Bundle = .main) {
        self.root = bundle.resourceURL?.appendingPathComponent(Self.embeddedDir, isDirectory: true)
    }

    func read(path: String) -> Data? {
        guard let root else { return nil }
        return try? Data(contentsOf: root.appendingPathComponent(path))
    }
}
