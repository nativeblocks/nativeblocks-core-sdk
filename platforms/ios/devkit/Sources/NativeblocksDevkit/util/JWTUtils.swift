import Foundation

internal class JWTUtils {
    static func decodedBody(_ encoded: String) -> [String: Any]? {
        do {
            let parts = encoded.split(separator: ".")
            guard parts.count > 1 else { return nil }
            let body = String(parts[1])
            let decodedData = Data.fromBase64(body)
            if decodedData != nil {
                let jsonObject = try JSONSerialization.jsonObject(with: decodedData!, options: [])
                return jsonObject as? [String: Any]
            }
        } catch {
            DevKitLogger.debug("JWTUtils", "Failed to decode JWT: \(error)")
        }
        return nil
    }
}

extension Data {
    /// Same as ``Data(base64Encoded:)``, but adds padding automatically
    /// (if missing, instead of returning `nil`).
    internal static func fromBase64(_ encoded: String) -> Data? {
        var encoded = encoded
        let remainder = encoded.count % 4
        if remainder > 0 {
            encoded = encoded.padding(
                toLength: encoded.count + 4 - remainder,
                withPad: "=", startingAt: 0)
        }

        return Data(base64Encoded: encoded)
    }
}
