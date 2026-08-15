import Combine
import Foundation
import UserNotifications

internal class LiveService: NSObject, UNUserNotificationCenterDelegate {
    private var viewModel: LiveViewModel!
    private static let TAG = "LiveService"
    private var notificationCenter: UNUserNotificationCenter

    init(viewModel: LiveViewModel, notificationCenter: UNUserNotificationCenter = .current()) {
        DevKitLogger.debug("LiveService", "init")
        self.viewModel = viewModel
        self.notificationCenter = notificationCenter
        super.init()
        self.notificationCenter.delegate = self

        viewModel.$uiState.sink { [weak self] state in
            self?.showNotification(state.notificationState)
        }
        .store(in: &viewModel.cancellables)

        requestNotificationPermission { granted in
            if granted {
                self.renewNotification()
            }
        }
    }

    static func start() {
        LiveModule.provideLiveService().onStartCommand(action: LiveServiceAction.start.rawValue)
    }

    func onStartCommand(action: String) {
        guard let actionType = LiveServiceAction(rawValue: action) else { return }
        switch actionType {
        case .auth:
            viewModel.updateAction(state: .auth)
        case .start:
            viewModel.updateAction(state: .start)
        case .connect:
            viewModel.updateAction(state: .connect)
        case .disconnect:
            viewModel.updateAction(state: .disconnect)
        case .logout:
            viewModel.updateAction(state: .logout)
        case .notificationDeleted:
            renewNotification()
        default:
            break
        }
    }

    private func renewNotification() {
        showNotification(viewModel.uiState.notificationState)
    }

    private func showNotification(_ state: NotificationState) {
        let content = UNMutableNotificationContent()
        content.title = DevKitResource.strings.DevKitTitle
        content.sound = .none

        var actions: [UNNotificationAction] = []

        switch state {
        case .authRequire:
            content.body = DevKitResource.strings.LiveServiceNotificationTapToAuth
            let authAction = UNNotificationAction(
                identifier: LiveServiceAction.auth.rawValue,
                title: DevKitResource.strings.LiveServiceNotificationAuth,
                options: .foreground
            )
            actions.append(authAction)

        case .connect:
            content.body = DevKitResource.strings.LiveServiceNotificationTapToDisconnect
            let disconnectAction = UNNotificationAction(
                identifier: LiveServiceAction.disconnect.rawValue,
                title: DevKitResource.strings.LiveServiceNotificationDisconnect,
                options: .foreground
            )
            actions.append(disconnectAction)

        case .notConnect:
            content.body = DevKitResource.strings.LiveServiceNotificationTapToStart
            let connectAction = UNNotificationAction(
                identifier: LiveServiceAction.connect.rawValue,
                title: DevKitResource.strings.LiveServiceNotificationConnect,
                options: .foreground
            )
            actions.append(connectAction)

        case .connecting:
            content.body = DevKitResource.strings.LiveServiceNotificationConnecting
        case .none:
            return
        }

        if state == .connect || state == .connecting || state == .notConnect {
            let logoutAction = UNNotificationAction(
                identifier: LiveServiceAction.logout.rawValue,
                title: DevKitResource.strings.LiveServiceNotificationLogout,
                options: .foreground
            )
            actions.append(logoutAction)
        }

        let stopAction = UNNotificationAction(
            identifier: LiveServiceAction.stop.rawValue,
            title: DevKitResource.strings.LiveServiceNotificationStop,
            options: .foreground
        )
        actions.append(stopAction)

        let dismissAction = UNNotificationAction(
            identifier: LiveServiceAction.notificationDeleted.rawValue,
            title: DevKitResource.strings.LiveServiceNotificationDismiss,
            options: .destructive)
        actions.append(dismissAction)

        let category = UNNotificationCategory(
            identifier: LiveServiceAction.notificationDeleted.rawValue,
            actions: actions,
            intentIdentifiers: [],
            options: .customDismissAction
        )
        notificationCenter.setNotificationCategories([category])
        content.categoryIdentifier = LiveServiceAction.notificationDeleted.rawValue

        let request = UNNotificationRequest(
            identifier: LiveServiceAction.notificationDeleted.rawValue,
            content: content,
            trigger: nil
        )

        notificationCenter.add(request) { error in
            if let error = error {
                DevKitLogger.debug(LiveService.TAG, "Error scheduling notification: \(error.localizedDescription)")
            }
        }
    }

    func handleNotificationAction(identifier: String) {
        DevKitLogger.debug(LiveService.TAG, "User interacted with notification: \(identifier)")
        if identifier == UNNotificationDefaultActionIdentifier {
            var action = ""
            switch viewModel.uiState.notificationState {
            case .authRequire:
                action = LiveServiceAction.auth.rawValue
            case .connect, .connecting:
                action = LiveServiceAction.disconnect.rawValue
            case .notConnect:
                action = LiveServiceAction.connect.rawValue
            case .none:
                return
            }

            onStartCommand(action: action)
        } else if identifier == UNNotificationDismissActionIdentifier {
            onStartCommand(action: LiveServiceAction.notificationDeleted.rawValue)
        } else {
            onStartCommand(action: identifier)
        }
    }

    private func requestNotificationPermission(completion: @escaping (Bool) -> Void) {
        notificationCenter.requestAuthorization(options: [.alert, .sound, .badge]) { granted, error in
            if let error = error {
                DevKitLogger.debug(LiveService.TAG, "Notification permission error: \(error.localizedDescription)")
                completion(false)
            } else if granted {
                DevKitLogger.debug(LiveService.TAG, "Notification permission granted.")
                completion(true)
            } else {
                completion(false)
                DevKitLogger.debug(LiveService.TAG, "Notification permission denied.")
            }
        }
    }

    func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        willPresent notification: UNNotification,
        withCompletionHandler completionHandler: @escaping (UNNotificationPresentationOptions) -> Void
    ) {
        completionHandler([.banner, .sound])
    }

    func userNotificationCenter(
        _ center: UNUserNotificationCenter, didReceive response: UNNotificationResponse,
        withCompletionHandler completionHandler: @escaping () -> Void
    ) {
        handleNotificationAction(identifier: response.actionIdentifier)
        completionHandler()
    }

    func cleanUp() {
        DevKitLogger.debug("LiveService", "cleanUp")
        viewModel.cancellables.forEach { $0.cancel() }
    }
}
