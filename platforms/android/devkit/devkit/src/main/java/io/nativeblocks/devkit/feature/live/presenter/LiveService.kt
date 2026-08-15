package io.nativeblocks.devkit.feature.live.presenter

import android.app.ActivityManager
import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.widget.Toast
import androidx.core.app.NotificationCompat
import androidx.core.content.ContextCompat
import androidx.lifecycle.LifecycleService
import androidx.lifecycle.ViewModelStore
import androidx.lifecycle.ViewModelStoreOwner
import androidx.lifecycle.lifecycleScope
import io.nativeblocks.devkit.DevKitInjector
import io.nativeblocks.devkit.R
import io.nativeblocks.devkit.util.DevKitLogger
import kotlinx.coroutines.flow.firstOrNull
import kotlinx.coroutines.launch
import org.koin.android.compat.ScopeCompat.getViewModel
import org.koin.core.annotation.KoinInternalApi

internal class LiveService : LifecycleService(), ViewModelStoreOwner {
    override val viewModelStore: ViewModelStore by lazy { ViewModelStore() }

    @OptIn(KoinInternalApi::class)
    private val viewModel by lazy {
        getViewModel(
            scope = DevKitInjector.get().koin.scopeRegistry.rootScope,
            owner = this,
            clazz = LiveViewModel::class.java,
        )
    }

    private lateinit var notificationManager: NotificationManager

    companion object {
        private val TAG = LiveService::class.java.simpleName
        private const val NOTIFICATION_ID = 1
        private const val CHANNEL_ID = "LiveKitScreenSharing"
        const val START_ACTION = "START"
        const val CONNECT_ACTION = "CONNECT"
        const val DISCONNECT_ACTION = "DISCONNECT"
        const val STOP_ACTION = "STOP"
        const val NOTIFICATION_DELETED = "NOTIFICATION_DELETED"
        const val LOGOUT_ACTION = "LOGOUT"

        fun start(context: Context) {
            DevKitLogger.d(TAG, "start")
            ContextCompat.startForegroundService(context, Intent(context, LiveService::class.java).apply {
                action = START_ACTION
            })
        }

        fun stop(context: Context) {
            DevKitLogger.d(TAG, "stop")
            if (isServiceRunning(context)) {
                ContextCompat.startForegroundService(context, Intent(context, LiveService::class.java).apply {
                    action = STOP_ACTION
                })
            }
        }

        private fun isServiceRunning(context: Context): Boolean {
            val activityManager = context.getSystemService(Context.ACTIVITY_SERVICE) as ActivityManager
            val services = activityManager.getRunningServices(Int.MAX_VALUE)

            for (runningService in services) {
                if (LiveService::class.java.name == runningService.service.className) {
                    return true
                }
            }
            return false
        }
    }

    override fun onCreate() {
        DevKitLogger.d(TAG, "onCreate")
        super.onCreate()
        notificationManager = getSystemService(NOTIFICATION_SERVICE) as NotificationManager
        createNotificationChannel()
        lifecycleScope.launch {
            viewModel.uiState.collect {
                showNotification(it.notificationState)
            }
        }
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        super.onStartCommand(intent, flags, startId)
        DevKitLogger.d(TAG, "onStartCommand action:${intent?.action}")
        when (intent?.action) {
            START_ACTION -> {
                viewModel.updateAction(Action.START)
            }

            CONNECT_ACTION -> {
                if (hasActiveActivity()) {
                    viewModel.updateAction(Action.CONNECT)
                } else {
                    Toast.makeText(this, getString(R.string.live_connect_action_open_app_error), Toast.LENGTH_SHORT)
                        .show()
                }
            }

            DISCONNECT_ACTION -> {
                viewModel.updateAction(Action.DISCONNECT)
            }

            STOP_ACTION -> {
                stopSelf()
            }

            LOGOUT_ACTION -> {
                viewModel.updateAction(Action.LOGOUT)
            }

            NOTIFICATION_DELETED -> {
                renewNotification()
            }
        }
        return START_STICKY
    }

    override fun onDestroy() {
        DevKitLogger.d(TAG, "onDestroy")
        super.onDestroy()
        viewModelStore.clear()
    }

    private fun renewNotification() {
        lifecycleScope.launch {
            viewModel.uiState.firstOrNull()?.let {
                showNotification(it.notificationState)
            }
        }
    }

    private fun showNotification(state: NotificationState) {
        val notification = createNotification(state)
        startForeground(NOTIFICATION_ID, notification)
    }

    private fun updateNotification(state: NotificationState) {
        val notification = createNotification(state)
        notificationManager.notify(NOTIFICATION_ID, notification)
    }

    private fun createNotification(state: NotificationState): Notification {
        val notificationLayout = LiveRemoteViews.createSmallRemoteView(this, state)
        val notificationLayoutExpanded = LiveRemoteViews.createLargeRemoteView(this, state)

        val deleteIntent = Intent(this, LiveService::class.java).apply {
            action = NOTIFICATION_DELETED
        }
        val deletePendingIntent = PendingIntent.getService(
            this,
            1,
            deleteIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        return NotificationCompat.Builder(this, CHANNEL_ID)
            .setStyle(NotificationCompat.DecoratedCustomViewStyle())
            .setCustomContentView(notificationLayout)
            .setCustomBigContentView(notificationLayoutExpanded)
            .setPriority(NotificationCompat.PRIORITY_MAX)
            .setSilent(true)
            .setOngoing(true)
            .setSmallIcon(R.drawable.ic_native_notification)
            .setDeleteIntent(deletePendingIntent)
            .build()
    }

    private fun createNotificationChannel() {
        val channel = NotificationChannel(
            CHANNEL_ID, getString(R.string.live_notification_channel_name), NotificationManager.IMPORTANCE_HIGH
        )
        val manager = getSystemService(NotificationManager::class.java)
        manager.createNotificationChannel(channel)
    }

    private fun hasActiveActivity(): Boolean {
        try {
            val am: ActivityManager = getSystemService(Context.ACTIVITY_SERVICE) as ActivityManager
            val allTasks = am.getRunningTasks(1)
            allTasks.forEach { aTask ->
                if (aTask.topActivity?.className?.contains(packageName) == true) {
                    DevKitLogger.d(TAG, "hasActiveActivity true task:$aTask")
                    return true
                }
            }
            DevKitLogger.d(TAG, "hasActiveActivity false")
            return false
        } catch (e: Exception) {
            DevKitLogger.e(TAG, "hasActiveActivity", e)
            return true
        }
    }
}