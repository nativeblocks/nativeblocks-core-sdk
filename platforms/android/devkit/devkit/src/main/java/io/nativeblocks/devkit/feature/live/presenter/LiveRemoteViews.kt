package io.nativeblocks.devkit.feature.live.presenter

import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.view.View
import android.widget.RemoteViews
import io.nativeblocks.devkit.R
import io.nativeblocks.devkit.feature.auth.presenter.AuthActivity

internal object LiveRemoteViews {
    fun createSmallRemoteView(context: Context, state: NotificationState): RemoteViews {
        val title =
            context.getString(R.string.live_notification_title) + if (state == NotificationState.CONNECTING) "..." else "   "

        val remoteView = RemoteViews(context.packageName, R.layout.notification_layout_small)
        remoteView.setTextViewText(R.id.notification_title, title)

        setupPendingIntent(context, remoteView, true)
        buttonVisibility(remoteView, state, true)

        return remoteView
    }

    fun createLargeRemoteView(context: Context, state: NotificationState): RemoteViews {
        val title = context.getString(R.string.live_notification_title)
        val description = when (state) {
            NotificationState.CONNECT -> context.getString(R.string.live_notification_connect)
            NotificationState.NOT_CONNECT -> context.getString(R.string.live_notification_not_connect)
            NotificationState.AUTH_REQUIRE -> context.getString(R.string.live_notification_auth)
            NotificationState.CONNECTING -> context.getString(R.string.live_notification_connecting)
        }

        val remoteView = RemoteViews(context.packageName, R.layout.notification_layout)
        remoteView.setTextViewText(R.id.notification_title, title)
        remoteView.setTextViewText(R.id.notification_text, description)

        setupPendingIntent(context, remoteView, false)
        buttonVisibility(remoteView, state, false)
        return remoteView
    }

    private fun setupPendingIntent(context: Context, remoteView: RemoteViews, isSmall: Boolean) {
        val connectIntent = Intent(context, LiveService::class.java).apply {
            action = LiveService.CONNECT_ACTION
        }
        val connectPendingIntent = PendingIntent.getService(
            context,
            0,
            connectIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )
        remoteView.setOnClickPendingIntent(R.id.button_connect, connectPendingIntent)

        val disconnectIntent = Intent(context, LiveService::class.java).apply {
            action = LiveService.DISCONNECT_ACTION
        }
        val disconnectPendingIntent = PendingIntent.getService(
            context,
            1,
            disconnectIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )
        remoteView.setOnClickPendingIntent(R.id.button_disconnect, disconnectPendingIntent)

        val authIntent = Intent(context, AuthActivity::class.java).apply {
            flags = Intent.FLAG_ACTIVITY_NO_ANIMATION
        }
        val authPendingIntent = PendingIntent.getActivity(
            context, 3, authIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )
        remoteView.setOnClickPendingIntent(R.id.button_auth, authPendingIntent)

        val stopIntent = Intent(context, LiveService::class.java).apply {
            action = LiveService.STOP_ACTION
        }

        val logoutIntent = Intent(context, LiveService::class.java).apply {
            action = LiveService.LOGOUT_ACTION
        }

        if (isSmall.not()) {
            val stopPendingIntent = PendingIntent.getService(
                context,
                -1,
                stopIntent,
                PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
            )
            remoteView.setOnClickPendingIntent(R.id.button_stop, stopPendingIntent)

            val logoutPendingIntent = PendingIntent.getService(
                context,
                -1,
                logoutIntent,
                PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
            )
            remoteView.setOnClickPendingIntent(R.id.button_logout, logoutPendingIntent)
        }
    }

    private fun buttonVisibility(
        remoteViews: RemoteViews,
        state: NotificationState,
        isSmall: Boolean
    ) {
        remoteViews.setViewVisibility(
            R.id.button_connect,
            if (state == NotificationState.NOT_CONNECT) View.VISIBLE else View.GONE
        )

        remoteViews.setViewVisibility(
            R.id.button_disconnect,
            if (state == NotificationState.CONNECT || state == NotificationState.CONNECTING) View.VISIBLE else View.GONE
        )

        remoteViews.setViewVisibility(
            R.id.button_auth,
            if (state == NotificationState.AUTH_REQUIRE) View.VISIBLE else View.GONE
        )

        if (isSmall.not()) {
            remoteViews.setViewVisibility(R.id.button_stop, View.VISIBLE)
            remoteViews.setViewVisibility(
                R.id.button_logout,
                if (state != NotificationState.AUTH_REQUIRE) View.VISIBLE else View.GONE
            )
        }
    }
}