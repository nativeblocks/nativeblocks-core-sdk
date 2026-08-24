import NativeblocksRuntimeFFI
import SwiftUI

internal struct NativeFrame: View {

    private let route: String
    private let args: [String: String]
    private let state: NativeblocksFrameState
    private let loading: () -> AnyView
    private let error: (String) -> AnyView
    private let instanceName: String

    @StateObject private var frameViewModel: FrameViewModel
    @Environment(\.verticalSizeClass) private var verticalSizeClass
    @Environment(\.horizontalSizeClass) private var horizontalSizeClass

    init(
        instanceName: String = "default",
        route: String,
        args: [String: String],
        state: NativeblocksFrameState = .stateless,
        loading: @escaping () -> AnyView,
        error: @escaping (String) -> AnyView
    ) {
        self.instanceName = instanceName
        self.route = route
        self.args = args
        self.state = state
        self.loading = loading
        self.error = error
        self._frameViewModel = StateObject(
            wrappedValue: NativeCoreSDKInjector.get(name: instanceName).makeFrameViewModel()
        )
    }

    var body: some View {
        Group {
            switch frameViewModel.renderingState {
            case .loading:
                loading()
            case .ready:
                RootLifecycle(instanceName: instanceName, vm: frameViewModel)
            case .error(let message):
                error(message)
            }
        }
        .task(id: FrameSetup(instanceName: instanceName, route: route, args: args, stateKey: state.key)) {
            frameViewModel.setupFrame(route: route, args: args, stateKey: state.key)
        }
        .onDisappear {
            frameViewModel.releaseFrame()
        }
    }
}

private struct FrameSetup: Equatable {
    let instanceName: String
    let route: String
    let args: [String: String]
    let stateKey: String?
}

private struct RootLifecycle: View {

    let instanceName: String
    @ObservedObject var vm: FrameViewModel

    var body: some View {
        Group {
            if let rootKey = vm.rootKey {
                Block(
                    instanceName: instanceName,
                    vm: vm,
                    blockKey: rootKey,
                    listItemIndex: NONE_INDEX,
                    parentScope: nil
                )
                    .task(id: vm.frameUpdateGeneration) {
                        vm.rootEntered(rootKey)
                    }
                    .onDisappear {
                        vm.rootExited(rootKey)
                    }
            } else {
                EmptyView()
            }
        }
    }
}

private struct Block: View {

    let instanceName: String
    @ObservedObject var vm: FrameViewModel
    let blockKey: String
    let listItemIndex: Int
    let parentScope: Any?

    var body: some View {
        if let block = vm.blockOf(blockKey) {
            let blockContext = makeBlockContext(for: block)
            if block.keyType == "ROOT" {
                AnyView(RootBlock(blockContext: blockContext))
            } else if let nativeBlock = vm.blockProvider.getProvidedBlocks()[block.keyType] {
                AnyView(nativeBlock(blockContext))
            } else if let fallbackBlock = vm.blockProvider.getFallbackBlock() {
                AnyView(fallbackBlock(block.keyType, blockKey))
                    .onAppear { vm.logBlockFallback(keyType: block.keyType, blockKey: blockKey) }
            } else {
                InternalFallbackBlock(key: block.keyType)
                    .onAppear { vm.logBlockFallback(keyType: block.keyType, blockKey: blockKey) }
            }
        } else {
            EmptyView()
        }
    }

    private func blockModifier(for block: NativeBlockModel) -> NativeblocksModifier {
        let items = block.modifiers
        if items.isEmpty {
            return .none
        }
        let provided = vm.modifierProvider.getProvidedModifiers()
        let instanceName = instanceName
        let listItemIndex = listItemIndex
        let parentScope = parentScope
        let vm = vm
        return NativeblocksModifier { content in
            var view = content
            for item in items {
                if let nativeModifier = provided[item.keyType] {
                    let modifierContext = ModifierContext(
                        instanceName: instanceName,
                        listItemIndex: listItemIndex,
                        onFindVariable: { data in
                            vm.variableOf(data?.value ?? "")?.value
                        },
                        onUpdateVariable: { data, value in
                            guard let data = data else { return }
                            vm.updateVariable(key: data.value, value: value)
                        },
                        onFindAction: { eventType in
                            vm.actionOf(blockKey: blockKey, eventType: eventType)
                        },
                        onHandleAction: { index, action, event in
                            vm.handleAction(index, action, event)
                        },
                        modifier: item,
                        scope: parentScope
                    )
                    view = nativeModifier(view, modifierContext)
                } else {
                    vm.logModifierFallback(keyType: item.keyType, blockKey: blockKey)
                }
            }
            return view
        }
    }

    private func makeBlockContext(for block: NativeBlockModel) -> BlockContext {
        return BlockContext(
            instanceName: instanceName,
            listItemIndex: listItemIndex,
            onFindVisibility: {
                vm.variableOf(block.visibility)?.value
            },
            onFindVariable: { data in
                vm.variableOf(data?.value ?? "")?.value
            },
            onUpdateVariable: { data, value in
                guard let data = data else { return }
                vm.updateVariable(key: data.value, value: value)
            },
            onFindAction: { eventType in
                vm.actionOf(blockKey: blockKey, eventType: eventType)
            },
            onHandleAction: { index, action, event in
                vm.handleAction(index, action, event)
            },
            block: block,
            modifier: blockModifier(for: block),
            onSubBlock: { blockKeys, subSlot, itemIndex, scope in
                return AnyView(
                    ForEach(blockKeys[subSlot.slot] ?? [], id: \.self) { childKey in
                        Block(
                            instanceName: instanceName,
                            vm: vm,
                            blockKey: childKey,
                            listItemIndex: itemIndex == NONE_INDEX ? listItemIndex : itemIndex,
                            parentScope: scope
                        )
                    }
                )
            }
        )
    }
}
