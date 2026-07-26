import Foundation

/// A singleton class responsible for managing and providing actions within the Nativeblocks SDK.
/// The `NativeActionProvider` class maintains a registry of actions that can be dynamically provided based on the action type.
internal class NativeActionProvider {
    /// A dictionary to store the actions, where the key is the action type and the value is the corresponding action implementation.
    private var actions = [String: any INativeAction]()

    /// Fallback action  implementation to be used when a specific action type is not found
    private var fallbackAction: ((String, String) -> Void)? = nil

    /// Contractors whose views must be mounted for their actions to work.
    private var actionsContractors = [any INativeActionContractor]()

    /// Registers a new action with the specified action key type.
    /// - Parameters:
    ///   - actionType: The key type for the action to be registered.
    ///   - action: The action instance that implements the `INativeAction` protocol.
    func provideAction(actionType: String, action: any INativeAction) {
        actions[actionType] = action
    }

    /// Retrieves the dictionary of all registered actions.
    /// - Returns: A dictionary containing all registered actions where the key is the action type and the value is the action instance.
    func getProvidedActions() -> [String: any INativeAction] {
        return actions
    }

    /// Sets the fallback action to be used when no matching action is registered or available.
    /// This action will be executed in place of unsupported or unrecognized actions.
    ///
    /// - Parameter block: A closure receiving the missing action's keyType and name.
    func onFallbackAction(block: @escaping (String, String) -> Void) {
        fallbackAction = block
    }

    /// Retrieves the fallback block.
    /// - Returns: Registered block
    func getFallbackAction() -> ((String, String) -> Void)? {
        return fallbackAction
    }

    /// Registers an action contractor.
    /// - Parameter actionContractor: The contractor to register.
    func provideActionContractor(_ actionContractor: any INativeActionContractor) {
        actionsContractors.append(actionContractor)
    }

    /// Retrieves every registered action contractor.
    func getProvidedActionContractors() -> [any INativeActionContractor] {
        return actionsContractors
    }

}
