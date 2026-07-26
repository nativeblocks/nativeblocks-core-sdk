import NativeblocksRuntimeFFI
import SwiftUI

private let ON_APPEAR = "onAppear"
private let ON_DISAPPEAR = "onDisappear"

internal struct NativeFrame: View {

    private let route: String
    private let args: [String: String]
    private let loading: () -> AnyView
    private let error: (String) -> AnyView

    @StateObject private var frameViewModel: FrameViewModel
    @Environment(\.verticalSizeClass) private var verticalSizeClass
    @Environment(\.horizontalSizeClass) private var horizontalSizeClass

    init(
        instance: String = "default",
        route: String,
        args: [String: String],
        loading: @escaping () -> AnyView,
        error: @escaping (String) -> AnyView
    ) {
        self.route = route
        self.args = args
        self.loading = loading
        self.error = error
        self._frameViewModel = StateObject(
            wrappedValue: NativeCoreSDKInjector.get(name: instance).makeFrameViewModel()
        )
    }

    var body: some View {
        Group {
            switch frameViewModel.renderingState {
            case .loading:
                loading()
            case .ready:
                RootLifecycle(vm: frameViewModel)
            case .error(let message):
                error(message)
            }
        }
        .environment(\.nativeWindowWidthClass, currentWindowWidthClass(verticalSizeClass, horizontalSizeClass))
        .onAppear {
            frameViewModel.setupFrame(route: route, args: args)
        }
        .onDisappear {
            frameViewModel.releaseFrame()
        }
    }
}

private struct RootLifecycle: View {

    @ObservedObject var vm: FrameViewModel

    var body: some View {
        Group {
            if let rootKey = vm.rootKey {
                Block(vm: vm, blockKey: rootKey, listItemIndex: NONE_INDEX)
                    .onAppear {
                        handle(rootKey, ON_APPEAR)
                    }
                    .onDisappear {
                        handle(rootKey, ON_DISAPPEAR)
                    }
                    .onChange(of: vm.frameUpdateGeneration) { _ in
                        handle(rootKey, ON_DISAPPEAR)
                        handle(rootKey, ON_APPEAR)
                    }
            } else {
                EmptyView()
            }
        }
    }

    private func handle(_ rootKey: String, _ event: String) {
        vm.handleAction(NONE_INDEX, vm.actionOf(blockKey: rootKey, eventType: event), event)
    }
}

private struct Block: View {

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
            instanceName: vm.instanceName,
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
