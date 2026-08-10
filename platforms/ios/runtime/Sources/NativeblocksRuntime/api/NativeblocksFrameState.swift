import Foundation

public enum NativeblocksFrameState {
    case stateless
    case stateful(key: String)

    internal var key: String? {
        switch self {
        case .stateless: return nil
        case .stateful(let key): return key
        }
    }
}
