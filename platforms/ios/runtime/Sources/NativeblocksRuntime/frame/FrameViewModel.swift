import Combine
import Foundation
import NativeblocksRuntimeFFI

@MainActor
internal final class FrameViewModel: ObservableObject {

    private let frameStateBridge: FrameStateBridge
    private let instanceName: String

    @Published private(set) var renderingState: RenderingState = .loading
    @Published private(set) var rootKey: String? = nil
    @Published private(set) var frameUpdateGeneration: Int = 0
    @Published private(set) var blocks: [String: NativeBlockModel] = [:]
    @Published private(set) var variables: [String: NativeVariableModel] = [:]

    private var actions: [String: [NativeActionModel]] = [:]
    private var setupTask: Task<Void, Never>?

    let blockProvider: NativeBlockProvider

    private lazy var actionTree = ActionTree(
        instanceName: instanceName,
        onFindVariable: { [weak self] key in
            self?.variables[key]
        },
        onFindBlock: { [weak self] key in
            self?.blocks[key]
        },
        onChangeBlock: { [weak self] blockKey, propertyKey, valueMobile, valueTablet, valueDesktop in
            self?.frameStateBridge.updateBlockProperty(
                blockKey: blockKey,
                propertyKey: propertyKey,
                valueMobile: valueMobile,
                valueTablet: valueTablet,
                valueDesktop: valueDesktop
            )
        },
        onVariableChange: { [weak self] variable in
            self?.updateVariable(key: variable.key, value: variable.value)
        }
    )

    init(frameStateBridge: FrameStateBridge, instanceName: String) {
        self.frameStateBridge = frameStateBridge
        self.instanceName = instanceName
        self.blockProvider = NativeBlockProviderRegistry.getOrCreate(instanceName)
    }

    func setupFrame(route: String, args: [String: String]) {
        setupTask?.cancel()
        setupTask = Task { [frameStateBridge] in
            await frameStateBridge.observeFrame(
                route: route,
                args: args,
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

    func blockOf(_ key: String) -> NativeBlockModel? {
        return blocks[key]
    }

    func actionOf(blockKey: String, eventType: String) -> NativeActionModel? {
        return actions[blockKey]?.first { $0.event == eventType }
    }

    func handleAction(_ index: Int, _ action: NativeActionModel?, _ performedEventType: String) {
        actionTree.handle(index: index, action: action, performedEventType: performedEventType)
    }

    func updateVariable(key: String, value: String) {
        frameStateBridge.updateVariable(key: key, value: value)
    }

    func releaseFrame() {
        setupTask?.cancel()
        setupTask = nil
        frameStateBridge.releaseFrame()
    }

    private func applyFull(_ frame: FrameFull) {
        variables = variables.filter { frame.variables.keys.contains($0.key) }
        syncVariables(frame.variables)
        blocks = blocks.filter { frame.blocks.keys.contains($0.key) }
        syncBlocks(frame.blocks)

        actions = frame.actions.mapValues { list in list.map { $0.toDomain() } }

        rootKey = frame.rootKey
        if frame.state == .ready {
            frameUpdateGeneration += 1
        }
        renderingState = frame.state
    }

    private func applyDiff(_ diff: FrameDiff) {
        if !diff.variables.isEmpty {
            syncVariables(diff.variables)
        }
        if !diff.blocks.isEmpty {
            syncBlocks(diff.blocks)
        }
    }

    private func syncVariables(_ runtimeVariables: [String: RuntimeFFIVariableModel]) {
        for (key, variable) in runtimeVariables {
            variables[key] = variable.toDomain()
        }
    }

    private func syncBlocks(_ runtimeBlocks: [String: RuntimeFFIBlockModel]) {
        for (key, block) in runtimeBlocks {
            blocks[key] = block.toDomain()
        }
    }
}
