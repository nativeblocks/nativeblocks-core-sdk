import Combine
import Foundation
import NativeblocksRuntimeFFI

private let ON_APPEAR = "onAppear"
private let ON_DISAPPEAR = "onDisappear"
private let ON_STATE_SAVED = "onStateSaved"
private let ON_STATE_RESTORED = "onStateRestored"

@MainActor
internal final class FrameViewModel: ObservableObject {

    private let frameStateBridge: FrameStateBridge
    private let instanceName: String

    @Published private(set) var renderingState: RenderingState = .loading
    @Published private(set) var rootKey: String? = nil
    @Published private(set) var frameUpdateGeneration: Int = 0
    
    @Published private(set) var variables: [String: NativeVariableModel] = [:]
    private(set) var blocks: [String: NativeBlockModel] = [:]
    private var actions: [String: [NativeActionModel]] = [:]
    
    private var setupTask: Task<Void, Never>?
    private var announcedGeneration = -1
    private var isAppeared = false
    private var isRestored = false
    private var isStateful = false

    let blockProvider: NativeBlockProvider
    let modifierProvider: NativeModifierProvider

    private lazy var actionTree = ActionTree(
        instanceName: instanceName,
        onFindVariable: { [weak self] key in
            self?.variables[key]
        },
        onVariableChange: { [weak self] variable in
            self?.updateVariable(key: variable.key, value: variable.value)
        },
        onLog: { [weak self] event in
            self?.frameStateBridge.logAction(event: event)
        }
    )

    init(frameStateBridge: FrameStateBridge, instanceName: String) {
        self.frameStateBridge = frameStateBridge
        self.instanceName = instanceName
        self.blockProvider = NativeBlockProviderRegistry.getOrCreate(instanceName)
        self.modifierProvider = NativeModifierProviderRegistry.getOrCreate(instanceName)
    }

    func setupFrame(route: String, args: [String: String], stateKey: String?) {
        setupTask?.cancel()
        isStateful = stateKey != nil
        setupTask = Task { [frameStateBridge] in
            await frameStateBridge.observeFrame(
                route: route,
                args: args,
                stateKey: stateKey,
                onFull: { [weak self] frame in
                    Task { @MainActor in self?.applyFull(frame) }
                },
                onDiff: { [weak self] diff in
                    Task { @MainActor in self?.applyDiff(diff) }
                }
            )
        }
    }

    func variableOf(_ key: String) -> NativeVariableModel? {
        return variables[key]
    }

    func valueOf(_ key: String) -> String? {
        return variables[key]?.value
    }

    func blockOf(_ key: String) -> NativeBlockModel? {
        return blocks[key]
    }

    func actionOf(blockKey: String, eventType: String) -> NativeActionModel? {
        let matches = actions[blockKey]?.filter { $0.event == eventType } ?? []
        if matches.count > 1 {
            frameStateBridge.logAction(
                event: .eventAmbiguous(event: eventType, blockKey: blockKey, count: Int32(matches.count))
            )
        }
        return matches.first
    }

    func logBlockFallback(keyType: String, blockKey: String) {
        frameStateBridge.logBlock(event: .blockFallback(keyType: keyType, blockKey: blockKey))
    }

    func logModifierFallback(keyType: String, blockKey: String) {
        frameStateBridge.logBlock(event: .modifierFallback(keyType: keyType, blockKey: blockKey))
    }

    func handleAction(_ index: Int, _ action: NativeActionModel?, _ performedEventType: String) {
        actionTree.handle(index: index, action: action, performedEventType: performedEventType)
    }

    func updateVariable(key: String, value: String) {
        frameStateBridge.updateVariable(key: key, value: value)
    }

    func rootEntered(_ rootKey: String) {
        isAppeared = true
        guard announcedGeneration != frameUpdateGeneration else {
            return
        }
        announcedGeneration = frameUpdateGeneration
        fireRootEvent(rootKey, isRestored ? ON_STATE_RESTORED : ON_APPEAR)
    }

    func rootExited(_ rootKey: String) {
        guard isAppeared else {
            return
        }
        isAppeared = false
        fireRootEvent(rootKey, isStateful ? ON_STATE_SAVED : ON_DISAPPEAR)
    }

    func fireRootEvent(_ rootKey: String, _ event: String) {
        handleAction(NONE_INDEX, actionOf(blockKey: rootKey, eventType: event), event)
    }

    func releaseFrame() {
        setupTask?.cancel()
        setupTask = nil
        frameStateBridge.releaseFrame()
    }

    private func applyFull(_ frame: FrameFull) {
        variables = variables.filter { frame.variables.keys.contains($0.key) }
        syncVariables(frame.variables)
        syncBlocks(frame.blocks)
        syncActions(frame.actions)

        rootKey = frame.rootKey
        isRestored = frame.restored
        if frame.state == .ready {
            frameUpdateGeneration += 1
        }
        renderingState = frame.state
    }

    private func applyDiff(_ diff: FrameDiff) {
        guard !diff.variables.isEmpty else { return }
        syncVariables(diff.variables)
    }

    private func syncVariables(_ runtimeVariables: [String: RuntimeFFIVariableModel]) {
        for (key, variable) in runtimeVariables {
            variables[key] = variable.toDomain()
        }
    }

    private func syncActions(_ runtimeActions: [String: [RuntimeFFIActionModel]]) {
        actions = runtimeActions.mapValues { list in list.map { $0.toDomain() } }
    }

    private func syncBlocks(_ runtimeBlocks: [String: RuntimeFFIBlockModel]) {
        blocks = runtimeBlocks.mapValues { $0.toDomain() }
    }
}
