package io.nativeblocks.runtime.api

import android.content.Context
import androidx.compose.runtime.Composable
import io.nativeblocks.runtime.api.provider.action.INativeAction
import io.nativeblocks.runtime.api.provider.action.INativeActionContractor
import io.nativeblocks.runtime.api.provider.action.NativeActionProviderRegistry
import io.nativeblocks.runtime.api.provider.block.BlockProps
import io.nativeblocks.runtime.api.provider.block.NativeBlockProviderRegistry
import io.nativeblocks.runtime.api.provider.kit.Kit
import io.nativeblocks.runtime.api.provider.logger.INativeLogger
import io.nativeblocks.runtime.api.provider.model.NativeScaffoldModel
import io.nativeblocks.runtime.api.provider.type.INativeType
import io.nativeblocks.runtime.api.provider.type.NativeTypeProvider
import io.nativeblocks.runtime.api.provider.type.NativeTypeProviderRegistry
import io.nativeblocks.runtime.di.NativeCoreSDKInjector
import io.nativeblocks.runtime.experiment.ExperimentUseCase
import io.nativeblocks.runtime.ffi.NativeRuntimeClientManager
import io.nativeblocks.runtime.ffi.disposeInstance
import io.nativeblocks.runtime.ffi.provideLogger
import io.nativeblocks.runtime.ffi.removeLogger
import io.nativeblocks.runtime.localization.LocalizationUseCase
import io.nativeblocks.runtime.logger.LoggerAdapter
import io.nativeblocks.runtime.scaffold.toHost
import io.nativeblocks.runtime.util.isValidInstanceName
import org.koin.core.qualifier.named
import kotlin.reflect.KClass

/**
 * Manages the lifecycle and configuration of the Nativeblocks framework.
 */
class NativeblocksManager internal constructor(
    private val name: String,
    private val context: Context,
    private val edition: NativeblocksEdition
) {

    companion object {
        /**
         * Singletons instance of the NativeblocksManager.
         */
        private val instanceRegistry = mutableMapOf<String, NativeblocksManager>()

        private val lock = Any()

        /**
         * Initializes the Nativeblocks framework.
         * @param applicationContext The application context.
         * @param edition The edition of Nativeblocks to initialize.
         */
        fun initialize(
            name: String = "default",
            applicationContext: Context,
            edition: NativeblocksEdition
        ): NativeblocksManager {
            synchronized(lock) {
                if (name.isValidInstanceName().not()) {
                    throw IllegalArgumentException("Please make sure the '$name' contains valid characters: A–Z, a–z, 0–9, _ or -")
                }
                if (!instanceRegistry.containsKey(name)) {
                    val manager = NativeblocksManager(name, applicationContext, edition)
                    instanceRegistry[name] = manager
                }
                return instanceRegistry[name]!!
            }
        }

        /**
         * Retrieves the singleton instance of the NativeblocksManager.
         * @return The NativeblocksManager instance.
         */
        fun getInstance(name: String = "default"): NativeblocksManager {
            return instanceRegistry[name]
                ?: throw IllegalStateException("NativeblocksManager '$name' has not been initialized.")
        }

        /**
         * Check whether the instance has been initialized or not
         * @return Boolean
         */
        fun isInitialized(name: String = "default"): Boolean {
            return instanceRegistry.containsKey(name)
        }
    }

    init {
        NativeCoreSDKInjector.init(this.name, context, edition)
    }

    private fun getKoin() = NativeCoreSDKInjector.get(this.name).koin
    private val typeProvider: NativeTypeProvider = NativeTypeProviderRegistry.getOrCreate(this.name)

    private val blockProvider get() = NativeBlockProviderRegistry.getOrCreate(this.name)
    private val actionProvider get() = NativeActionProviderRegistry.getOrCreate(this.name)
    private val actionContractors = mutableListOf<INativeActionContractor>()
    private val loggerTypes = mutableSetOf<String>()
    private val kits = mutableListOf<Kit>()

    internal fun providedActionContractors(): List<INativeActionContractor> = actionContractors.toList()

    /**
     * Provides a block implementation.
     * @param blockType The type of the block.
     * @param block The block implementation to register.
     * @return The NativeblocksManager instance for chaining.
     */
    fun provideBlock(
        blockType: String,
        block: @Composable (blockProps: BlockProps) -> Unit
    ): NativeblocksManager {
        blockProvider.provideBlock(blockType, block)
        return this
    }

    /**
     * Provides a fallback block to be used when a requested block keyType is not found.
     * @param block A composable lambda that receives the missing block's keyType and key.
     * @return The NativeblocksManager instance for chaining.
     */
    fun provideFallbackBlock(block: @Composable (keyType: String, key: String) -> Unit): NativeblocksManager {
        blockProvider.onFallbackBlock(block)
        return this
    }

    /**
     * Provides an action implementation.
     * @param actionType The type of the action.
     * @param action The action implementation to register.
     * @return The NativeblocksManager instance for chaining.
     */
    fun provideAction(actionType: String, action: INativeAction): NativeblocksManager {
        actionProvider.provideAction(actionType, action)
        return this
    }

    /**
     * Provides an action contractor.
     * @param actionContractor The action contractor to provide.
     * @return The NativeblocksManager instance for chaining.
     */
    fun provideActionContractor(actionContractor: INativeActionContractor): NativeblocksManager {
        actionContractors.add(actionContractor)
        return this
    }

    /**
     * Provides a fallback action to be used when a requested action keyType is not found.
     * @param block A lambda that receives the missing action's keyType and name.
     * @return The NativeblocksManager instance for chaining.
     */
    fun provideFallbackAction(block: (keyType: String, name: String) -> Unit): NativeblocksManager {
        actionProvider.onFallbackAction(block)
        return this
    }

    /**
     * Provides an event logger.
     * @param loggerType The type of the logger.
     * @param logger The logger implementation to provide.
     * @return The NativeblocksManager instance for chaining.
     */
    fun provideEventLogger(loggerType: String, logger: INativeLogger): NativeblocksManager {
        runCatching {
            provideLogger(this.name, loggerType, LoggerAdapter(logger))
            loggerTypes.add(loggerType)
        }
        return this
    }

    /**
     * Provides a kit.
     * @param kit The kit to provide.
     * @return The NativeblocksManager instance for chaining.
     */
    fun provideKit(kit: Kit): NativeblocksManager {
        runCatching {
            kit.attach(this.context, this.name, this.edition)
            kits.add(kit)
        }
        return this
    }

    /**
     * Synchronizes the specified frame with the server.
     * @param route The route of the frame to synchronize.
     * @return The NativeblocksManager instance for chaining.
     */
    suspend fun syncFrame(route: String): NativeblocksManager {
        val runtimeClient: NativeRuntimeClientManager by getKoin().inject(named(this.name))
        runCatching { runtimeClient.frameClient.syncFrame(route, emptyMap()) }
        return this
    }

    /**
     * clear all frames from cache.
     * @return The NativeblocksManager instance for chaining.
     */
    suspend fun clearAllFrames(): NativeblocksManager {
        val runtimeClient: NativeRuntimeClientManager by getKoin().inject(named(this.name))
        runCatching { runtimeClient.frameClient.clearAll(emptyList()) }
        return this
    }

    /**
     * clear frame from cache.
     * @param route The route of the frame to clear.
     * @return The NativeblocksManager instance for chaining.
     */
    suspend fun clearFrame(route: String): NativeblocksManager {
        val runtimeClient: NativeRuntimeClientManager by getKoin().inject(named(this.name))
        runCatching { runtimeClient.frameClient.clear(route) }
        return this
    }

    /**
     * Throws away the frame kept under [key], so the next visit starts fresh.
     * @param key The key given to [NativeblocksFrameState.Stateful].
     */
    fun clearFrameState(key: String): NativeblocksManager {
        val runtimeClient: NativeRuntimeClientManager by getKoin().inject(named(this.name))
        runtimeClient.frameClient.clearFrameState(key)
        return this
    }

    /**
     * Throws away every kept frame.
     */
    fun clearAllFrameStates(): NativeblocksManager {
        val runtimeClient: NativeRuntimeClientManager by getKoin().inject(named(this.name))
        runtimeClient.frameClient.clearAllFrameStates()
        return this
    }

    /**
     * Sets the language for the frames.
     * @param languageCode The ISO language code to set as the current language (e.g. "EN", "FA")
     * @return The NativeblocksManager instance for chaining.
     */
    fun setLanguage(languageCode: String): NativeblocksManager {
        val localizationUseCase: LocalizationUseCase by getKoin().inject(named(this.name))
        localizationUseCase.setLocalization(languageCode = languageCode)
        return this
    }

    /**
     * Translate a key in provided language.
     * @param key The translation key
     * @return The translated value or null.
     */
    fun translate(key: String): String? {
        val localizationUseCase: LocalizationUseCase by getKoin().inject(named(this.name))
        return localizationUseCase.translate(key).getOrNull()
    }

    /**
     * Retrieves the scaffold model for the current configuration.
     */
    suspend fun getScaffold(): Result<NativeScaffoldModel> {
        val runtimeClient: NativeRuntimeClientManager by getKoin().inject(named(this.name))
        return runCatching { runtimeClient.scaffoldClient.getScaffold().toHost() }
    }

    /**
     * Retrieves the experiment value for the provided key as type [T].
     *
     * Supports the following types:
     *  - STRING: String
     *  - NUMBER: Int, Float, Double, Long
     *  - BOOLEAN: Boolean
     *  - JSON: String (as raw JSON)
     *
     * @param key The identifier for the experiment value.
     * @param defaultValue The default value to use if key not found or conversion fails.
     * @param cacheTTL Cache time-to-live in milliseconds. Use 0 for no caching (always fetch fresh).
     *                 Default is 24 hours (86,400,000 ms).
     * @return The experiment value as type [T] or [defaultValue] if unavailable or conversion fails.
     *
     * @sample
     * ```
     * // Use default cache (24 hours)
     * val showNewUI = getExperiment("show_new_ui", false)
     *
     * // No cache - always fetch fresh
     * val criticalFlag = getExperiment("payment_enabled", false, 0L)
     *
     * // Custom cache duration (10 minutes)
     * val theme = getExperiment("theme_color", "#FF0000", 10 * 60 * 1000L)
     * ```
     */
    @Suppress("UNCHECKED_CAST")
    suspend fun <T : Any> getExperiment(key: String, defaultValue: T, cacheTTL: Long = 24 * 60 * 60 * 1000): T {
        val experimentUseCase: ExperimentUseCase by getKoin().inject(named(this.name))
        val result = experimentUseCase.get(key, cacheTTL) ?: return defaultValue
        val (value, variableType) = result

        val requestedType = when (defaultValue) {
            is String -> if (variableType == "JSON") "JSON" else "STRING"
            is Int, is Float, is Double, is Long -> "NUMBER"
            is Boolean -> "BOOLEAN"
            else -> null
        }
        if (requestedType == null || variableType != requestedType) {
            return defaultValue
        }

        return try {
            when (variableType) {
                "STRING" -> value as T
                "NUMBER" -> when (defaultValue) {
                    is Int -> value.toIntOrNull() as? T ?: defaultValue
                    is Float -> value.toFloatOrNull() as? T ?: defaultValue
                    is Double -> value.toDoubleOrNull() as? T ?: defaultValue
                    is Long -> value.toLongOrNull() as? T ?: defaultValue
                    else -> defaultValue
                }
                "BOOLEAN" -> when (defaultValue) {
                    is Boolean -> value.toBooleanStrictOrNull() as? T ?: defaultValue
                    else -> defaultValue
                }
                "JSON" -> value as T
                else -> defaultValue
            }
        } catch (_: Exception) {
            defaultValue
        }
    }

    /**
     * Sets global parameters for the frames.
     * @param @parameters Vararg of pairs representing global parameters, where each pair consists of a key and a value.
     * @return The NativeblocksManager instance for chaining.
     */
    fun setGlobalParameters(vararg parameters: Pair<String, String>): NativeblocksManager {
        return setGlobalParameters(parameters.toMap())
    }

    /**
     * Sets global parameters for the frames.
     * @param @parameters Vararg of pairs representing global parameters, where each pair consists of a key and a value.
     * @return The NativeblocksManager instance for chaining.
     */
    fun setGlobalParameters(parameters: Map<String, String>): NativeblocksManager {
        val runtimeClient: NativeRuntimeClientManager by getKoin().inject(named(this.name))
        runCatching { runtimeClient.globalParameterClient.set(parameters) }
        return this
    }

    /**
     * Provides a type converter for a given type within the NativeblocksManager.
     *
     * @param T The type for which the converter is being provided.
     * @param type The `KClass` of the type.
     * @param converter The implementation of `INativeType` to handle conversion for the type.
     * @return The instance of `NativeblocksManager` to allow method chaining.
     */
    fun <T : Any> provideTypeConverter(
        type: KClass<T>,
        converter: INativeType<T>
    ): NativeblocksManager {
        typeProvider.provideTypeConverter(type, converter)
        return this
    }

    /**
     * Retrieves the type converter for a given type from the NativeblocksManager.
     *
     * @param T The type for which the converter is being retrieved.
     * @param type The `KClass` of the type.
     * @return The `INativeType` implementation for the requested type.
     * @throws NullPointerException If no converter has been provided for the requested type.
     */
    fun <T : Any> getTypeConverter(type: KClass<T>): INativeType<T> {
        return typeProvider.getTypeConverter(type)
    }

    /**
     * Cleans up resources and destroys the NativeblocksManager instance.
     */
    fun destroy() {
        kits.forEach { runCatching { it.detach(this.name) } }
        kits.clear()
        loggerTypes.forEach { runCatching { removeLogger(this.name, it) } }
        loggerTypes.clear()
        runCatching {
            val runtimeClient: NativeRuntimeClientManager by getKoin().inject(named(this.name))
            runtimeClient.close()
        }
        NativeCoreSDKInjector.destroy(this.name)
        runCatching { disposeInstance(this.name) }
        instanceRegistry.remove(this.name)
        NativeBlockProviderRegistry.remove(this.name)
        NativeActionProviderRegistry.remove(this.name)
        NativeTypeProviderRegistry.remove(this.name)
        actionContractors.clear()
    }

}