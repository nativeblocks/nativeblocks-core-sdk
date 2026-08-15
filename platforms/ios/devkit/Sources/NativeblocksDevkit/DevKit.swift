import Foundation
import NativeblocksRuntime
import UserNotifications

/// `DevKit` provides live features such as real-time updates, remote logging and screen
/// management for Nativeblocks applications.
/// It is specifically designed to work with the Nativeblocks Studio environment.
///
/// Example:
/// ```swift
/// let devKit = DevKit.Builder()
///     .keepScreenOn()
///     .autoConnect()
///     .logTracking()
///     .build()
///
/// NativeblocksManager.getInstance(name: instanceName).provideKit(devKit)
/// ```
public final class DevKit: Kit {

    private static var configuredInstanceName: String?

    // MARK: - Properties
    /// Keeps the screen active and prevents it from locking.
    private let keepScreenOn: Bool

    /// Automatically connects to the environment during setup.
    private let autoConnect: Bool

    /// Enables remote logging to Nativeblocks Studio.
    private let logTracking: Bool

    /// Reference to the dev logger instance
    private var devLogger: DevLogger?

    // MARK: - Initialization

    /// Private initializer to enforce Builder pattern usage.
    /// - Parameters:
    ///   - keepScreenOn: A flag to keep the screen active (default: `false`).
    ///   - autoConnect: A flag to enable automatic connection (default: `false`).
    ///   - logTracking: A flag to enable remote logging (default: `false`).
    private init(keepScreenOn: Bool, autoConnect: Bool, logTracking: Bool) {
        self.keepScreenOn = keepScreenOn
        self.autoConnect = autoConnect
        self.logTracking = logTracking
    }

    /// Attaches `DevKit` to the given instance based on the provided `NativeblocksEdition`.
    ///
    /// - Parameters:
    ///   - instanceName: The unique identifier for this DevKit instance.
    ///   - edition: The `NativeblocksEdition` to configure the environment.
    ///
    /// - Warning: This function should not be called directly by developers. It is invoked automatically
    /// during the initialization process of Nativeblocks.
    public func attach(instanceName: String, edition: NativeblocksEdition) {
        if let configured = Self.configuredInstanceName, configured != instanceName {
            fatalError("DevKit is already set up with '\(configured)'. Cannot reconfigure with '\(instanceName)'.")
        }
        if Self.configuredInstanceName == nil {
            Self.configuredInstanceName = instanceName
        }

        DevKitLogger.debug("DevKit", "attach")
        let environment: DevKitEnvironment

        switch edition {
        case .cloud(_, let apiKey, let developmentMode):
            if !developmentMode {
                fatalError("DevKit doesn't support production mode, please disable it or use on the developmentMode")
            }
            environment = DevKitEnvironment(
                instanceName: instanceName,
                apiKey: apiKey,
                screenSharing: false,
                autoConnect: autoConnect
            )
        default:
            fatalError("DevKit is supported only for cloud edition")
        }

        if keepScreenOn {
            LockScreen.lockScreenOff()
        }

        DevKitInjector.initialize(environment: environment)

        if logTracking {
            devLogger = LoggingModule.provideDevLogger()
            if let logger = devLogger {
                _ = NativeblocksManager.getInstance(name: instanceName)
                    .provideEventLogger(loggerType: "DevKit:\(instanceName)", logger: logger)
            }
        }

        LiveService.start()
    }

    /// Manually authorizes a user with the DevKit module.
    ///
    /// This method is used to authenticate a user with a backend endpoint and a valid token and real-time connection endpoint.
    /// It should be called before using any DevKit features that require authentication.
    /// - Parameters:
    ///   - endpoint: The base URL of your backend API.
    ///   - token: The user authentication token (e.g., JWT).
    ///   - realtimeEndpoint: The real-time server endpoint used for live features (e.g., logs, screen sharing).
    public func userAuthorization(endpoint: String, token: String, realtimeEndpoint: String) {
        DevKitInjector.userAuthorization(endpoint: endpoint, token: token, realtimeEndpoint: realtimeEndpoint)
    }

    /// Manual user logout
    public func userLogout() {
        DevKitInjector.userLogout()
    }

    /// Detaches the `DevKit` instance and cleans up resources.
    /// - Note: If `keepScreenOn` is enabled, the screen lock will be re-enabled.
    public func detach(instanceName: String) {
        DevKitLogger.debug("DevKit", "detach")
        if keepScreenOn {
            LockScreen.lockScreenOn()
        }
        devLogger = nil
        DevKitInjector.destroy()
        Self.configuredInstanceName = nil
    }

    // MARK: - Builder

    /// Builder class for constructing `DevKit` instances with a fluent API.
    ///
    /// Example:
    /// ```swift
    /// let devKit = DevKit.Builder()
    ///     .keepScreenOn()
    ///     .autoConnect()
    ///     .logTracking()
    ///     .build()
    /// ```
    public class Builder {
        private var _keepScreenOn: Bool = false
        private var _autoConnect: Bool = false
        private var _logTracking: Bool = false

        public init() {}

        /// Enables the keep screen on feature.
        /// When enabled, the device screen will stay on and won't auto-lock.
        /// - Returns: The builder instance for method chaining.
        @discardableResult
        public func keepScreenOn() -> Builder {
            self._keepScreenOn = true
            return self
        }

        /// Enables automatic connection to the development environment.
        /// When enabled, the SDK will automatically connect to Nativeblocks Studio on startup.
        /// - Returns: The builder instance for method chaining.
        @discardableResult
        public func autoConnect() -> Builder {
            self._autoConnect = true
            return self
        }

        /// Enables remote logging to Nativeblocks Studio.
        /// When enabled, application logs will be sent to Studio for debugging.
        /// - Returns: The builder instance for method chaining.
        @discardableResult
        public func logTracking() -> Builder {
            self._logTracking = true
            return self
        }

        /// Builds and returns the configured `DevKit` instance.
        /// - Returns: A new `DevKit` instance with the configured options.
        public func build() -> DevKit {
            return DevKit(
                keepScreenOn: _keepScreenOn,
                autoConnect: _autoConnect,
                logTracking: _logTracking
            )
        }
    }
}
