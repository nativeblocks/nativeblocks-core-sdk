import Foundation

extension String {

    internal func isValidInstanceName() -> Bool {
        let pattern = "^[A-Za-z0-9_-]+$"
        let regex = try! NSRegularExpression(pattern: pattern)
        let range = NSRange(location: 0, length: self.utf16.count)
        return !self.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty && regex.firstMatch(in: self, options: [], range: range) != nil
    }

}
