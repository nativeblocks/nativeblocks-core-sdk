internal struct AuthUIState {
    var hasPermission: Bool = false
    var showCamera: Bool = false

    static func initState() -> AuthUIState {
        return AuthUIState(hasPermission: false, showCamera: false)
    }
}

internal enum AuthUIAction {
    case onScan(qrData: String)
    case onPermissionRequest
    case onShowCamera
    case onClose
    case copyFromClipboard(data: String?)
}
