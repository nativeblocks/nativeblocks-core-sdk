import Foundation
import NativeblocksRuntimeFFI

extension String {

    internal func isValidInstanceName() -> Bool {
        return NativeblocksRuntimeFFI.isValidInstanceName(name: self)
    }

}
