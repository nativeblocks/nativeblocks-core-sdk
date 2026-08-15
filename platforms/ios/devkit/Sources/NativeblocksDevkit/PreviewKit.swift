import Foundation
import Combine
import NativeblocksRuntime

#if os(iOS)
import UIKit
#endif

/// `PreviewKit` provides A/B testing and release preview features for Nativeblocks applications.
/// It allows runtime modification of global parameters through a parameter form UI.
///
/// **Important**: PreviewKit only works in production mode (when `developmentMode = false`).
///
/// Use `PreviewKit.Builder` to create and configure an instance.
///
/// Example:
/// ```swift
/// let previewKit = PreviewKit.Builder()
///     .launchOnShake()
///     .build()
///
/// NativeblocksManager.getInstance(name: "myInstance")
///     .provideKit(previewKit)
///
/// // Manual launch is always available
/// previewKit.launch()
/// ```
public final class PreviewKit: Kit {

    // MARK: - Static Properties

    private static var configuredInstanceName: String?
    private static let lock = NSLock()


    private let shakeToLaunch: Bool
    private var shakeDetector: ShakeDetector?
    private var cancellables = Set<AnyCancellable>()

    private init(shakeToLaunch: Bool) {
        self.shakeToLaunch = shakeToLaunch
    }

    /// Attaches PreviewKit to the given instance name with the provided edition.
    ///
    /// - Parameters:
    ///   - instanceName: The instance name for this PreviewKit configuration.
    ///   - edition: The `NativeblocksEdition` to configure the environment.
    ///
    /// - Warning: This function should not be called directly by developers. It is invoked automatically
    /// during the initialization process of Nativeblocks.
    ///
    /// - Throws: `fatalError` if PreviewKit is already set up with a different instance,
    ///   if development mode is used (only production mode is supported),
    ///   or if the Community edition is used (only Cloud edition is supported).
    public func attach(instanceName: String, edition: NativeblocksEdition) {
        Self.lock.lock()
        defer { Self.lock.unlock() }

        if let existing = Self.configuredInstanceName, existing != instanceName {
            fatalError("PreviewKit is already set up with '\(existing)'. Cannot reconfigure with '\(instanceName)'.")
        }

        if Self.configuredInstanceName == nil {
            Self.configuredInstanceName = instanceName
        }

        let environment: PreviewKitEnvironment

        switch edition {
        case .cloud(_, _, let developmentMode):
            if developmentMode {
                fatalError("PreviewKit doesn't support development mode. Use DevKit for development.")
            }
            environment = PreviewKitEnvironment(instanceName: instanceName)

        default:
            fatalError("PreviewKit is supported only for Cloud edition")
        }

        PreviewKitInjector.shared.initialize(environment: environment)

        #if os(iOS)
        if shakeToLaunch {
            shakeDetector = ShakeDetector()
            shakeDetector?.onShakeDetected
                .sink { [weak self] in
                    self?.launchInternal()
                }
                .store(in: &cancellables)

            shakeDetector?.start()
        }
        #endif

        DevKitLogger.debug("PreviewKit", "attach completed for instance: \(instanceName)")
    }


    /// Detaches the PreviewKit instance and cleans up resources.
    ///
    /// Stops the shake detector (if enabled), clears the context reference, and resets the instance name.
    public func detach(instanceName: String) {
        DevKitLogger.debug("PreviewKit", "detach")

        shakeDetector?.stop()
        shakeDetector = nil
        cancellables.removeAll()
        PreviewKitInjector.shared.destroy()

        Self.lock.lock()
        Self.configuredInstanceName = nil
        Self.lock.unlock()
    }

    /// Launches the parameter form UI.
    ///
    /// Call this method to manually show the parameter form.
    /// This works regardless of whether shake detection is enabled.
    ///
    /// - Throws: `fatalError` if called before `attach(instanceName:edition:)`.
    public func launch() {
        guard Self.configuredInstanceName != nil else {
            fatalError("PreviewKit must be attached before launch()")
        }
        launchInternal()
    }

    private func launchInternal() {
        guard let instanceName = Self.configuredInstanceName else { return }

        #if os(iOS)
        DispatchQueue.main.async {
            ParameterFormPresenter.shared.present(instanceName: instanceName)
        }
        #endif
    }

    /// Builder class for creating `PreviewKit` instances with a fluent API.
    ///
    /// Example:
    /// ```swift
    /// let previewKit = PreviewKit.Builder()
    ///     .launchOnShake()
    ///     .build()
    /// ```
    public class Builder {

        private var shakeToLaunch: Bool = false

        public init() {}

        /// Enables shake detection to auto-launch the parameter form UI.
        ///
        /// When enabled, shaking the device will automatically show the parameter form UI.
        /// Manual launch via `PreviewKit.launch()` is always available regardless of this setting.
        ///
        /// Default: disabled
        ///
        /// - Returns: This builder instance for method chaining.
        @discardableResult
        public func launchOnShake() -> Builder {
            self.shakeToLaunch = true
            return self
        }

        /// Builds and returns a `PreviewKit` instance with the configured settings.
        ///
        /// - Returns: A new `PreviewKit` instance.
        public func build() -> PreviewKit {
            return PreviewKit(shakeToLaunch: shakeToLaunch)
        }
    }
}
