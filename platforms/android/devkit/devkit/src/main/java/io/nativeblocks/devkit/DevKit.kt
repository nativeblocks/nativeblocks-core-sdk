package io.nativeblocks.devkit

import android.content.Context
import androidx.activity.ComponentActivity
import io.nativeblocks.runtime.api.NativeblocksEdition
import io.nativeblocks.runtime.api.NativeblocksManager
import io.nativeblocks.runtime.api.provider.kit.Kit
import io.nativeblocks.devkit.feature.live.presenter.LiveService
import io.nativeblocks.devkit.feature.logging.DevLogger
import io.nativeblocks.devkit.lib.permission.NotificationPermissionContractor
import io.nativeblocks.devkit.lib.permission.KeepScreenOnContractor
import io.nativeblocks.devkit.session.DevKitEnvironment
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job

/**
 * Devkit provides live features such as real-time updates, screen sharing,
 * notification permissions, and screen management for Nativeblocks applications.
 *
 * Use [DevKit.Builder] to create and configure an instance.
 *
 * Example:
 * ```
 * // In your Activity
 * val devkit = DevKit.Builder(this)
 *     .keepScreenOn()
 *     .autoConnect()
 *     .logTracking()
 *     .build()
 *
 * NativeblocksManager.getInstance(instanceName).provideKit(devkit)
 * ```
 *
 * DevKit automatically handles notification permission requests using the provided Activity.
 * When permission is required, DevKit will launch the system permission dialog.
 */
class DevKit private constructor(
    private val activity: ComponentActivity,
    private val keepScreenOn: Boolean,
    private val autoConnect: Boolean,
    private val logTracking: Boolean,
) : Kit {

    companion object {
        private var configuredInstanceName: String? = null
    }

    private var job = Job()
    private var coroutineScope = CoroutineScope(Dispatchers.IO + job)
    private val keepScreenOnContractor = KeepScreenOnContractor()
    private val notificationPermission = NotificationPermissionContractor(activity)
    private var devLogger: DevLogger? = null

    /**
     * Sets up Devkit with the given [context] and [edition].
     *
     * @param context The Android [android.content.Context] used to initialize the live service.
     * @param edition The [NativeblocksEdition] to configure the environment.
     * @param instanceName The instance name for this Devkit configuration.
     *
     * @throws IllegalStateException if Devkit is already set up with a different instance.
     * @throws IllegalArgumentException if production mode is used (only development mode is supported).
     * @throws IllegalAccessException if the Community edition is used (only Cloud edition is supported).
     */
    override fun attach(context: Context, instanceName: String, edition: NativeblocksEdition) {
        synchronized(this) {
            if (configuredInstanceName != null && configuredInstanceName != instanceName) {
                throw IllegalStateException("DevKit is already set up with '$configuredInstanceName'. Cannot reconfigure with '$instanceName'.")
            }
            if (configuredInstanceName == null) {
                configuredInstanceName = instanceName
            }
        }

        job = Job()
        coroutineScope = CoroutineScope(Dispatchers.IO + job)

        val environment = when (edition) {
            is NativeblocksEdition.Cloud -> {
                if (!edition.developmentMode) {
                    throw IllegalArgumentException("Devkit doesn't support production mode, please disable it or use development mode")
                }
                DevKitEnvironment(
                    instanceName = instanceName,
                    apiKey = edition.apiKey,
                    screenSharing = false,
                    autoConnect = autoConnect,
                )
            }

            is NativeblocksEdition.Community -> {
                throw IllegalAccessException("Devkit is supported only for cloud edition")
            }
        }

        DevKitInjector.init(context, environment)

        if (logTracking) {
            devLogger = DevKitInjector.get().koin.get<DevLogger>()
            devLogger?.let {
                NativeblocksManager.getInstance(instanceName).provideEventLogger("DevKit:$instanceName", it)
            }
        }

        if (notificationPermission.isPermissionRequired()) {
            notificationPermission.requestPermission { isGranted ->
                if (isGranted) {
                    LiveService.start(context)
                }
            }
        } else {
            LiveService.start(context)
        }

        if (keepScreenOn) {
            NativeblocksManager.getInstance(instanceName).provideActionContractor(keepScreenOnContractor)
        }
    }

    /**
     * Authenticates the user manually by providing essential credentials and endpoints required
     * for establishing a secure and authenticated connection to the Nativeblocks cloud service.
     *
     * @param endpoint The base URL for your Nativeblocks backend.
     * @param token A valid user authentication token issued by the backend.
     * @param realtimeEndpoint The WebSocket endpoint for real-time interactions such as live updates or signaling.
     *
     * @throws IllegalArgumentException If any of the parameters are empty or malformed.
     */
    suspend fun userAuthorization(endpoint: String, token: String, realtimeEndpoint: String) {
        DevKitInjector.userAuthorization(endpoint, token, realtimeEndpoint)
    }

    /**
     * Logs out the currently authorized user manually by clearing all sensitive tokens
     * and connection data from memory.
     */
    suspend fun userLogout() {
        DevKitInjector.userLogout()
    }

    /**
     * Cleans up resources and cancels any active operations associated with Devkit.
     */
    override fun detach(instanceName: String) {
        job.cancel()
        devLogger = null
        notificationPermission.unregister()
        DevKitInjector.destroy()
        configuredInstanceName = null
    }

    /**
     * Builder class for creating [DevKit] instances with a fluent API.
     *
     * @param activity The [ComponentActivity] used for permission requests.
     *
     * Example:
     * ```
     * val devkit = DevKit.Builder(this)
     *     .keepScreenOn()
     *     .autoConnect()
     *     .build()
     * ```
     */
    class Builder(
        private val activity: ComponentActivity
    ) {
        private var keepScreenOn: Boolean = false
        private var autoConnect: Boolean = false
        private var logTracking: Boolean = false

        /**
         * Sets whether to keep the device screen on during live sessions.
         *
         * @return This builder instance for method chaining.
         */
        fun keepScreenOn() = apply {
            this.keepScreenOn = true
        }

        /**
         * Sets whether to automatically connect to the live service.
         *
         * @return This builder instance for method chaining.
         */
        fun autoConnect() = apply {
            this.autoConnect = true
        }

        /**
         * Enables logging to transmit log events to Nativeblocks Studio.
         *
         * @return This builder instance for method chaining.
         */
        fun logTracking() = apply {
            this.logTracking = true
        }

        /**
         * Builds and returns a [DevKit] instance with the configured settings.
         *
         * @return A new [DevKit] instance.
         */
        fun build(): DevKit {
            return DevKit(
                activity = activity,
                keepScreenOn = keepScreenOn,
                autoConnect = autoConnect,
                logTracking = logTracking,
            )
        }
    }
}