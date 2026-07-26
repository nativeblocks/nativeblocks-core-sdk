package io.nativeblocks.runtime.api.provider.action

/**
 * Provides and manages actions and action contractors within the native framework.
 */
internal class NativeActionProvider {

    /**
     * Map of action types to their corresponding handlers.
     */
    private val actions = mutableMapOf<String, INativeAction>()

    /**
     * List of action contractors managed by the provider.
     */
    private val actionsContractors: MutableList<INativeActionContractor> = mutableListOf()

    /**
     * Fallback action to be executed if no specific action is found.
     */
    private var fallbackAction: ((keyType: String, name: String) -> Unit)? = null

    /**
     * Registers a new action handler for a specific action type.
     * @param actionType The type of the action.
     * @param action The action handler to register.
     */
    fun provideAction(actionType: String, action: INativeAction) {
        actions[actionType] = action
    }

    /**
     * Retrieves all registered action handlers.
     * @return A map of action types to their handlers.
     */
    fun getProvidedActions(): Map<String, INativeAction> {
        return actions.toMap()
    }

    /**
     * Registers a new action contractor.
     * @param actionContractor The action contractor to register.
     */
    fun provideActionContractor(actionContractor: INativeActionContractor) {
        actionsContractors.add(actionContractor)
    }

    /**
     * Retrieves all registered action contractors.
     * @return A list of registered action contractors.
     */
    fun getProvidedActionContractors(): MutableList<INativeActionContractor> {
        return actionsContractors
    }

    /**
     * Registers a fallback action to be executed if no specific action is found.
     * @param action The fallback action handler to register.
     */
    fun onFallbackAction(action: (keyType: String, name: String) -> Unit) {
        fallbackAction = action
    }

    /**
     * Retrieves the registered fallback action.
     * @return The fallback action handler, or null if none is registered.
     */
    fun getFallbackAction(): ((keyType: String, name: String) -> Unit)? {
        return fallbackAction
    }

}
