import Foundation
import NativeblocksRuntimeFFI

internal protocol FrameStateBridge {
    func observeFrame(
        route: String,
        args: [String: String],
        stateKey: String?,
        onFull: @escaping (FrameFull) -> Void,
        onDiff: @escaping (FrameDiff) -> Void
    ) async

    func updateVariable(key: String, value: String)


    func logAction(event: ActionLogEvent)

    func logBlock(event: BlockLogEvent)

    func releaseFrame()
}

internal final class FrameStateBridgeImpl: FrameStateBridge {

    private let frameStateManager: FrameStateManager

    init(frameStateManager: FrameStateManager) {
        self.frameStateManager = frameStateManager
    }

    func observeFrame(
        route: String,
        args: [String: String],
        stateKey: String?,
        onFull: @escaping (FrameFull) -> Void,
        onDiff: @escaping (FrameDiff) -> Void
    ) async {
        await frameStateManager.setupFrame(
            route: route,
            args: args,
            stateKey: stateKey,
            observer: FrameStateObserverAdapter(onFull: onFull, onDiff: onDiff)
        )
    }

    func updateVariable(key: String, value: String) {
        frameStateManager.updateVariable(key: key, value: value)
    }


    func logAction(event: ActionLogEvent) {
        frameStateManager.logAction(event: event)
    }

    func logBlock(event: BlockLogEvent) {
        frameStateManager.logBlock(event: event)
    }

    func releaseFrame() {
        frameStateManager.release()
    }
}

private final class FrameStateObserverAdapter: FrameStateObserver, @unchecked Sendable {

    private let onFull: (FrameFull) -> Void
    private let onDiff: (FrameDiff) -> Void

    init(onFull: @escaping (FrameFull) -> Void, onDiff: @escaping (FrameDiff) -> Void) {
        self.onFull = onFull
        self.onDiff = onDiff
    }

    func onFrameChange(change: FrameChangeType) {
        switch change {
        case .full(let frame): onFull(frame)
        case .diff(let frame): onDiff(frame)
        }
    }
}
