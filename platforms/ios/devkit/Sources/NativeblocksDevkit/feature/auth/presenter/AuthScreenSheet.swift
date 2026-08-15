import Combine
import Foundation
import SwiftUI

internal struct AuthScreenSheet {
    static func present(onDismiss: (() -> Void)? = nil) {
        #if os(iOS)
            guard let windowScene = UIApplication.shared.connectedScenes.first as? UIWindowScene else { return }
            guard let rootViewController = windowScene.windows.first?.rootViewController else { return }
            let viewModel = AuthModule.provideAuthViewModel()

            let authScreen = UIHostingController(rootView: AuthScreen(viewModel: viewModel))
            authScreen.modalPresentationStyle = .pageSheet

            if let sheet = authScreen.sheetPresentationController {
                sheet.detents = [.medium()]
                sheet.prefersGrabberVisible = true
                sheet.delegate = rootViewController as? UISheetPresentationControllerDelegate
            }

            viewModel.setOnDismiss {
                authScreen.dismiss(animated: true) {
                    onDismiss?()
                }
            }

            viewModel.setOnMessage { title, message in
                let alert = UIAlertController(title: title, message: message, preferredStyle: .alert)
                authScreen.present(alert, animated: true, completion: nil)
                Timer.scheduledTimer(withTimeInterval: 3.0, repeats: false, block: { _ in alert.dismiss(animated: true, completion: nil) })
            }

            rootViewController.present(authScreen, animated: true) {
                if let onDismiss = onDismiss {
                    authScreen.presentationController?.delegate = DismissDelegate(onDismiss: onDismiss)
                }
            }

        #elseif os(macOS)
            let authScreen = NSHostingController(rootView: AuthScreen(viewModel: AuthModule.provideAuthViewModel()))
            if let keyWindow = NSApplication.shared.windows.first {
                keyWindow.contentViewController?.presentAsModalWindow(authScreen)
            }
        #endif
    }
}
