import AVFoundation
import Combine
import SwiftUI

final internal class AuthViewModel: ObservableObject {
    private var authRepository: AuthRepository
    private let TAG = "AuthViewModel"
    init(authRepository: AuthRepository) {
        self.authRepository = authRepository
    }

    @Published var uiState = AuthUIState.initState()
    private var cancellables = Set<AnyCancellable>()
    private var onDismiss: (() -> Void)? = nil
    private var onMessage: ((_ title: String, _ message: String) -> Void)? = nil

    func setOnDismiss(onDismiss: @escaping () -> Void) {
        self.onDismiss = onDismiss
    }

    func setOnMessage(onMessage: @escaping (_ title: String, _ message: String) -> Void) {
        self.onMessage = onMessage
    }

    func sendAction(_ action: AuthUIAction) {
        switch action {
        case .onScan(let qrData):
            DevKitLogger.debug(TAG, "Scanned QR Data: \(qrData)")

            if authRepository.updateTokenAndEndpoint(qrData: qrData) {
                onDismiss?()
            } else {
                DevKitLogger.debug(TAG, "Data is not valid please try again")
                onMessage?("Scane failed", "Data is not valid please try again")
            }
        case .onPermissionRequest:
            requestCameraPermission()
            uiState.hasPermission = true
        case .onShowCamera:
            uiState.showCamera = true
        case .copyFromClipboard(let data):
            DevKitLogger.debug(TAG, "Copy From Clipboard: \(data ?? "")")
            if data?.isEmpty ?? true {
                onMessage?("Copy", "Failed: Data is not valid please try again")
            } else {
                if authRepository.updateTokenAndEndpoint(qrData: data!) {
                    onDismiss?()
                } else {
                    DevKitLogger.debug(TAG, "Data is not valid please try again")
                    onMessage?("Copy", "Failed: Data is not valid please try again")
                }
            }
        case .onClose:
            // Handle closing logic if needed.
            break
        }
    }

    private func checkCameraPermission() {
        switch AVCaptureDevice.authorizationStatus(for: .video) {
        case .authorized:
            uiState.hasPermission = true
        case .notDetermined:
            requestCameraPermission()
        case .denied, .restricted:
            uiState.hasPermission = false
        @unknown default:
            uiState.hasPermission = false
        }
    }

    private func requestCameraPermission() {
        AVCaptureDevice.requestAccess(for: .video) { granted in
            DispatchQueue.main.async {
                self.uiState.hasPermission = granted
            }
        }
    }
}
