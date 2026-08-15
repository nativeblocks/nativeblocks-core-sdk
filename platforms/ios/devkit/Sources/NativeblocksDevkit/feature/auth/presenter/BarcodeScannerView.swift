import AVFoundation
import SwiftUI

#if canImport(AppKit)
    import AppKit
#endif

internal struct BarcodeScannerView: View {
    var onScan: (String) -> Void

    var body: some View {
        #if os(iOS)
            BarcodeScannerViewControllerRepresentable(onScan: onScan)
        #else
            MacOSBarcodeScannerView(onScan: onScan)
        #endif
    }
}

#if os(iOS)
    internal struct BarcodeScannerViewControllerRepresentable: UIViewControllerRepresentable {
        var onScan: (String) -> Void

        func makeUIViewController(context: Context) -> BarcodeScannerViewController {
            let viewController = BarcodeScannerViewController()
            viewController.onScan = onScan
            return viewController
        }

        func updateUIViewController(_ uiViewController: BarcodeScannerViewController, context: Context) {}
    }

    internal class BarcodeScannerViewController: UIViewController, AVCaptureMetadataOutputObjectsDelegate {
        var captureSession: AVCaptureSession!
        var previewLayer: AVCaptureVideoPreviewLayer!
        var onScan: ((String) -> Void)?

        override func viewDidLoad() {
            super.viewDidLoad()

            captureSession = AVCaptureSession()

            guard let videoCaptureDevice = AVCaptureDevice.default(for: .video) else { return }
            let videoInput: AVCaptureDeviceInput

            do {
                videoInput = try AVCaptureDeviceInput(device: videoCaptureDevice)
            } catch {
                return
            }

            if captureSession.canAddInput(videoInput) {
                captureSession.addInput(videoInput)
            } else {
                return
            }

            let metadataOutput = AVCaptureMetadataOutput()

            if captureSession.canAddOutput(metadataOutput) {
                captureSession.addOutput(metadataOutput)

                metadataOutput.setMetadataObjectsDelegate(self, queue: DispatchQueue.main)
                metadataOutput.metadataObjectTypes = [.qr]
            } else {
                return
            }

            previewLayer = AVCaptureVideoPreviewLayer(session: captureSession)
            previewLayer.frame = view.layer.bounds
            previewLayer.videoGravity = .resizeAspectFill
            view.layer.addSublayer(previewLayer)

            captureSession.startRunning()
        }

        override func viewWillLayoutSubviews() {
            super.viewWillLayoutSubviews()
            if let previewLayer = previewLayer {
                previewLayer.frame = view.layer.bounds
            }
        }

        func metadataOutput(
            _ output: AVCaptureMetadataOutput, didOutput metadataObjects: [AVMetadataObject], from connection: AVCaptureConnection
        ) {
            if let metadataObject = metadataObjects.first {
                guard let readableObject = metadataObject as? AVMetadataMachineReadableCodeObject else { return }
                guard let stringValue = readableObject.stringValue else { return }
                AudioServicesPlaySystemSound(SystemSoundID(kSystemSoundID_Vibrate))
                onScan?(stringValue)
            }

        }

        override func viewWillDisappear(_ animated: Bool) {
            super.viewWillDisappear(animated)

            if captureSession.isRunning {
                captureSession.stopRunning()
            }
        }

        deinit {
            if captureSession.isRunning {
                captureSession.stopRunning()
            }
        }
    }
#endif

#if os(macOS)
    internal struct MacOSBarcodeScannerView: NSViewRepresentable {
        var onScan: (String) -> Void

        func makeNSView(context: Context) -> NSView {
            let view = NSView()
            // Placeholder view for macOS implementation.
            // macOS does not have an easy equivalent to AVCaptureSession, so you'd need to use a third-party library
            // or implement your own QR code scanning logic using Vision framework.
            return view
        }

        func updateNSView(_ nsView: NSView, context: Context) {}
    }
#endif
