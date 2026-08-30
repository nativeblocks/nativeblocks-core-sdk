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
                    parentScope: nil,
                    resolver: nil
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
    var resolver: TemplateResolver? = nil

    var body: some View {
        if let block = vm.blockOf(blockKey), vm.valueOf(block.visibility) != "false" {
            let blockContext = makeBlockContext(for: block)
            if block.keyType == "ROOT" {
                AnyView(RootBlock(blockContext: blockContext))
            } else if case .rendering(let render) = vm.blockProvider.getProvidedBlocks()[block.keyType] {
                AnyView(render(blockContext))
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
        let resolver = resolver
        let vm = vm
        return NativeblocksModifier { content in
            var view = content
            for item in items {
                if let nativeModifier = provided[item.keyType] {
                    let modifierContext = ModifierContext(
                        instanceName: instanceName,
                        listItemIndex: listItemIndex,
                        onFindVariable: { data in
                            vm.valueOf(data?.value ?? "")
                        },
                        onUpdateVariable: { data, value in
                            guard let data = data else { return }
                            vm.updateVariable(key: data.value, value: value)
                        },
                        onFindAction: { eventType in
                            vm.actionOf(blockKey: blockKey, eventType: eventType)
                        },
                        onHandleAction: { index, action, event in
                            vm.handleAction(index, action, event, parentScope)
                        },
                        modifier: item,
                        scope: parentScope,
                        resolveTemplate: { resolver?.resolve($0) ?? $0 }
                    )
                    view = nativeModifier(view, modifierContext)
                } else {
                    vm.logModifierFallback(keyType: item.keyType, blockKey: blockKey)
                }
            }
            return view
        }
    }

    private func makeBlockContext(for block: NativeBlockModel, resolver: TemplateResolver? = nil) -> BlockContext {
        let resolver = resolver ?? self.resolver
        return BlockContext(
            instanceName: instanceName,
            listItemIndex: listItemIndex,
            onFindVariable: { data in
                vm.valueOf(data?.value ?? "")
            },
            onUpdateVariable: { data, value in
                guard let data = data else { return }
                vm.updateVariable(key: data.value, value: value)
            },
            onFindAction: { eventType in
                vm.actionOf(blockKey: blockKey, eventType: eventType)
            },
            onHandleAction: { index, action, event in
                vm.handleAction(index, action, event, parentScope)
            },
            block: block,
            modifier: blockModifier(for: block),
            onDescribeSubBlock: { blockKeys, subSlot, describeScope in
                return AnyView(
                    ForEach(blockKeys[subSlot.slot] ?? [], id: \.self) { childKey in
                        if let child = vm.blockOf(childKey),
                            case .describing(let describe) = vm.blockProvider.getProvidedBlocks()[child.keyType] {
                            AnyView(
                                describe(
                                    makeBlockContext(
                                        for: child,
                                        resolver: (describeScope as? TemplateResolver) ?? resolver
                                    ),
                                    describeScope
                                )
                            )
                        } else {
                            Color.clear.frame(width: 0, height: 0)
                                .onAppear { vm.logBlockFallback(keyType: block.keyType, blockKey: childKey) }
                        }
                    }
                )
            },
            onSubBlock: { blockKeys, subSlot, itemIndex, scope in
                return AnyView(
                    ForEach(blockKeys[subSlot.slot] ?? [], id: \.self) { childKey in
                        Block(
                            instanceName: instanceName,
                            vm: vm,
                            blockKey: childKey,
                            listItemIndex: itemIndex == NONE_INDEX ? listItemIndex : itemIndex,
                            parentScope: scope,
                            resolver: (scope as? TemplateResolver) ?? resolver
                        )
                    }
                )
            },
            scope: parentScope,
            resolveTemplate: { resolver?.resolve($0) ?? $0 }
        )
    }
}
