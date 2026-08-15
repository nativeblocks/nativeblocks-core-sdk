package io.nativeblocks.devkit

import android.content.Context
import android.content.Intent
import io.nativeblocks.runtime.api.NativeblocksEdition
import io.nativeblocks.runtime.api.provider.kit.Kit
import io.nativeblocks.devkit.feature.preview.presenter.ParameterFormActivity
import io.nativeblocks.devkit.util.ShakeDetector
import io.nativeblocks.devkit.util.ShakeDetectorCallback
import io.nativeblocks.devkit.session.PreviewKitEnvironment
import java.lang.ref.WeakReference

/**
 * PreviewKit provides A/B testing and release preview features for Nativeblocks applications.
 * It allows runtime modification of global parameters through a parameter form UI.
 *
 * **Important**: PreviewKit only works in production mode (when developmentMode = false).
 *
 * Use [PreviewKit.Builder] to create and configure an instance.
 *
 * Example:
 * ```
 * val previewKit = PreviewKit.Builder()
 *     .launchOnShake()
 *     .build()
 *
 * NativeblocksManager.getInstance(instanceName).provideKit(previewKit)
 *
 * // Manual launch is always available
 * previewKit.launch()
 * ```
 */
class PreviewKit private constructor(
    private val shakeToLaunch: Boolean
) : Kit {

    companion object {
        private var configuredInstanceName: String? = null
    }

    private var shakeDetector: ShakeDetector? = null
    private var contextRef: WeakReference<Context>? = null

    /**
     * Sets up PreviewKit with the given [context] and [edition].
     *
     * @param context The Android [Context] used to initialize the shake detector and UI.
     * @param instanceName The instance name for this PreviewKit configuration.
     * @param edition The [NativeblocksEdition] to configure the environment.
     *
     * @throws IllegalStateException if PreviewKit is already set up with a different instance.
     * @throws IllegalArgumentException if development mode is used (only production mode is supported).
     * @throws IllegalAccessException if the Community edition is used (only Cloud edition is supported).
     */
    override fun attach(context: Context, instanceName: String, edition: NativeblocksEdition) {
        synchronized(this) {
            if (configuredInstanceName != null && configuredInstanceName != instanceName) {
                throw IllegalStateException("PreviewKit is already set up with '$configuredInstanceName'. Cannot reconfigure with '$instanceName'.")
            }
            if (configuredInstanceName == null) {
                configuredInstanceName = instanceName
            }
        }

        val environment = when (edition) {
            is NativeblocksEdition.Cloud -> {
                if (edition.developmentMode) {
                    throw IllegalArgumentException("PreviewKit doesn't support development mode. Use DevKit for development.")
                }
                PreviewKitEnvironment(instanceName = instanceName)
            }

            is NativeblocksEdition.Community -> {
                throw IllegalAccessException("PreviewKit is supported only for Cloud edition")
            }
        }

        this.contextRef = WeakReference(context)

        PreviewKitInjector.init(context, environment)

        if (shakeToLaunch) {
            shakeDetector = ShakeDetector(
                context = context,
                callback = object : ShakeDetectorCallback {
                    override fun onShakeDetected() {
                        launch()
                    }
                }
            )
            shakeDetector?.start()
        }
    }

    /**
     * Launches the parameter form UI.
     *
     * Call this method to manually show the parameter form.
     * This works regardless of whether shake detection is enabled.
     *
     * @throws IllegalStateException if called before [attach].
     */
    fun launch() {
        val context = contextRef?.get()
            ?: throw IllegalStateException("PreviewKit.setup() must be called before launch()")

        val intent = Intent(context, ParameterFormActivity::class.java).apply {
            putExtra(ParameterFormActivity.EXTRA_INSTANCE_NAME, configuredInstanceName)
            addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
        }
        context.startActivity(intent)
    }

    override fun detach(instanceName: String) {
        shakeDetector?.stop()
        shakeDetector = null
        PreviewKitInjector.destroy()
        configuredInstanceName = null
        contextRef = null
    }

    /**
     * Builder class for creating [PreviewKit] instances with a fluent API.
     *
     * Example:
     * ```
     * val previewKit = PreviewKit.Builder()
     *     .launchOnShake()
     *     .build()
     * ```
     */
    class Builder {
        private var shakeToLaunch: Boolean = false

        /**
         *
         * When shaking the device will automatically show the parameter form UI.
         * Manual launch via [PreviewKit.launch] is always available regardless of this setting.
         *
         * Default: `true`
         *
         * @return This builder instance for method chaining.
         */
        fun launchOnShake() = apply {
            this.shakeToLaunch = true
        }

        /**
         * Builds and returns a [PreviewKit] instance with the configured settings.
         *
         * @return A new [PreviewKit] instance.
         */
        fun build(): PreviewKit {
            return PreviewKit(shakeToLaunch = shakeToLaunch)
        }
    }
}