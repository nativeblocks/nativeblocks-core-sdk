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
        .environment(\.nativeWindowWidthClass, currentWindowWidthClass(verticalSizeClass, horizontalSizeClass))
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
                Block(instanceName: instanceName, vm: vm, blockKey: rootKey, listItemIndex: NONE_INDEX)
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

    var body: some View {
        if let block = vm.blockOf(blockKey) {
            let props = makeBlockProps(for: block)
            if block.keyType == "ROOT" {
                AnyView(RootBlock(blockProps: props))
            } else if let nativeBlock = vm.blockProvider.getProvidedBlocks()[block.keyType] {
                AnyView(nativeBlock(props))
            } else if let fallbackBlock = vm.blockProvider.getFallbackBlock() {
                AnyView(fallbackBlock(block.keyType, blockKey))
            } else {
                InternalFallbackBlock(key: block.keyType)
            }
        } else {
            EmptyView()
        }
    }

    private func makeBlockProps(for block: NativeBlockModel) -> BlockProps {
        return BlockProps(
            instanceName: instanceName,
            listItemIndex: listItemIndex,
            onFindVariable: { key in
                vm.variableOf(key)
            },
            onVariableChange: { variable in
                vm.updateVariable(key: variable.key, value: variable.value)
            },
            onFindAction: { eventType in
                vm.actionOf(blockKey: blockKey, eventType: eventType)
            },
            onHandleAction: { index, action, event in
                vm.handleAction(index, action, event)
            },
            block: block,
            onSubBlock: { blockKeys, subSlot, itemIndex, _ in
                AnyView(
                    ForEach(blockKeys[subSlot.slot] ?? [], id: \.self) { childKey in
                        Block(
                            instanceName: instanceName,
                            vm: vm,
                            blockKey: childKey,
                            listItemIndex: itemIndex == NONE_INDEX ? listItemIndex : itemIndex
                        )
                    }
                )
            }
        )
    }
}
