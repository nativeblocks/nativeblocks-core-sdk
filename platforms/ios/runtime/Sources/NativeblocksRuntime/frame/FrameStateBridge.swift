import Foundation
import NativeblocksRuntimeFFI

internal protocol FrameStateBridge {
    func observeFrame(
        route: String,
        args: [String: String],
        onFull: @escaping (FrameFull) -> Void,
        onDiff: @escaping (FrameDiff) -> Void
    ) async

    func updateVariable(key: String, value: String)

    func updateBlockProperty(
        blockKey: String,
        propertyKey: String,
        valueMobile: String,
        valueTablet: String,
        valueDesktop: String
    )

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
        onFull: @escaping (FrameFull) -> Void,
        onDiff: @escaping (FrameDiff) -> Void
    ) async {
        await frameStateManager.setupFrame(
            route: route,
            args: args,
            observer: FrameStateObserverAdapter(onFull: onFull, onDiff: onDiff)
        )
    }

    func updateVariable(key: String, value: String) {
        frameStateManager.updateVariable(key: key, value: value)
    }

    func updateBlockProperty(
        blockKey: String,
        propertyKey: String,
        valueMobile: String,
        valueTablet: String,
        valueDesktop: String
    ) {
        frameStateManager.updateBlockProperty(
            blockKey: blockKey,
            propertyKey: propertyKey,
            valueMobile: valueMobile,
            valueTablet: valueTablet,
            valueDesktop: valueDesktop
        )
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
