import Combine
import SwiftUI

internal struct AuthScreen: View {
    @ObservedObject var viewModel: AuthViewModel
    var body: some View {
        AuthScreenContentView(uiState: viewModel.uiState) { authUIAction in
            viewModel.sendAction(authUIAction)
        }
    }
}

internal struct AuthScreenContentView: View {
    var uiState: AuthUIState
    let sendAction: (AuthUIAction) -> Void

    var copyfromClipBoardView: some View {
        Button(action: {
            let clipboardValue: String? = {
                #if os(iOS)
                UIPasteboard.general.string
                #elseif os(macOS)
                NSPasteboard.general.string(forType: .string)
                #endif
            }()
            sendAction(.copyFromClipboard(data: clipboardValue ?? ""))
        }) {
            Text(DevKitResource.strings.AuthCopyFromClipboard)
                .frame(maxWidth: .infinity)
                .padding()
                .background(Color.white)
                .foregroundColor(.blue)
                .cornerRadius(8).overlay(
                    RoundedRectangle(cornerRadius: 8)
                        .stroke(Color.blue, lineWidth: 2)
                )
        }
    }

    var body: some View {
        VStack {
            Spacer()
            VStack {
                Text(DevKitResource.strings.AuthScreenGetStarted)
                    .font(.system(size: 24, weight: .bold))
                    .padding(.top, 16)
                Spacer()
                if !uiState.hasPermission {
                    VStack {
                        Text(DevKitResource.strings.AuthScreenCameraAccessRequired)
                            .font(.system(size: 14))
                            .padding(.vertical, 8)
                            .multilineTextAlignment(.center)

                        Button(action: {
                            sendAction(.onPermissionRequest)
                        }) {
                            Text(DevKitResource.strings.AuthScreenEnableCameraAccess)
                                .frame(maxWidth: .infinity)
                                .padding()
                                .background(Color.blue)
                                .foregroundColor(.white)
                                .cornerRadius(8)

                        }

                        copyfromClipBoardView
                            .padding(.top, 12)
                    }
                    .padding()
                } else {
                    if uiState.showCamera {
                        BarcodeScannerView(onScan: { qrData in
                            sendAction(.onScan(qrData: qrData))
                        })
                        .frame(width: 200, height: 200)
                        .cornerRadius(16)

                        copyfromClipBoardView
                            .padding(.top, 12)
                    } else {
                        Button(action: {
                            sendAction(.onShowCamera)
                        }) {
                            Text(DevKitResource.strings.AuthScreenStartScanning)
                                .frame(maxWidth: .infinity)
                                .padding()
                                .background(Color.blue)
                                .foregroundColor(.white)
                                .cornerRadius(8)
                        }
                        copyfromClipBoardView
                            .padding(.top, 12)
                    }
                }
                Spacer()
            }
            .cornerRadius(16)
            .padding()
        }
        .background(Color.clear)
        .edgesIgnoringSafeArea(.all)
        .onAppear {
            sendAction(.onPermissionRequest)
        }
    }
}

internal struct AuthScreen_Previews: PreviewProvider {
    static var previews: some View {
        AuthScreenContentView(uiState: AuthUIState.initState()) { _ in }
    }
}

internal struct AuthScreen_HasPermission_Previews: PreviewProvider {
    static var previews: some View {
        AuthScreenContentView(uiState: AuthUIState.init(hasPermission: true)) { _ in }
    }
}
