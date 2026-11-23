package com.wirewave.android.service

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.Service
import android.content.Context
import android.content.Intent
import android.content.pm.ServiceInfo
import android.net.wifi.WifiManager
import android.os.Build
import android.os.IBinder
import android.os.PowerManager
import com.wirewave.android.R
import com.wirewave.core.config.AppConfig

/**
 * Foreground service keeping the WireWave connection alive across app switches,
 * screen-off, and Doze using a connectedDevice foreground type, WifiLock, and partial WakeLock.
 */
class WireWaveConnectionService : Service() {
    private var wifiLock: WifiManager.WifiLock? = null
    private var wakeLock: PowerManager.WakeLock? = null

    override fun onBind(intent: Intent?): IBinder? = null

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        when (intent?.action) {
            ACTION_STOP -> {
                releaseLocks()
                stopForeground(STOP_FOREGROUND_REMOVE)
                stopSelf()
                return START_NOT_STICKY
            }
            else -> {
                val pcName = intent?.getStringExtra(EXTRA_PC_NAME) ?: "PC"
                val microphoneActive = intent?.getBooleanExtra(EXTRA_MICROPHONE_ACTIVE, false) == true
                acquireLocks()
                val notification = buildNotification(pcName, microphoneActive)
                try {
                    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
                        val serviceTypes = ServiceInfo.FOREGROUND_SERVICE_TYPE_CONNECTED_DEVICE or
                            if (microphoneActive) ServiceInfo.FOREGROUND_SERVICE_TYPE_MICROPHONE else 0
                        startForeground(
                            NOTIFICATION_ID,
                            notification,
                            serviceTypes
                        )
                    } else {
                        startForeground(NOTIFICATION_ID, notification)
                    }
                } catch (_: SecurityException) {
                    if (microphoneActive) {
                        releaseLocks()
                        stopSelf()
                        return START_NOT_STICKY
                    }
                    startForeground(NOTIFICATION_ID, notification)
                }
                return START_STICKY
            }
        }
    }

    override fun onDestroy() {
        releaseLocks()
        super.onDestroy()
    }

    private fun acquireLocks() {
        if (wifiLock == null) {
            val wifiMgr = applicationContext.getSystemService(Context.WIFI_SERVICE) as? WifiManager
            wifiLock = wifiMgr?.createWifiLock(WifiManager.WIFI_MODE_FULL_LOW_LATENCY, "WireWave:Link")?.apply {
                setReferenceCounted(false)
                acquire()
            }
        }
        if (wakeLock == null) {
            val powerMgr = getSystemService(Context.POWER_SERVICE) as? PowerManager
            wakeLock = powerMgr?.newWakeLock(PowerManager.PARTIAL_WAKE_LOCK, "WireWave:Session")?.apply {
                setReferenceCounted(false)
                acquire(12 * 60 * 60 * 1000L)
            }
        }
    }

    private fun releaseLocks() {
        wifiLock?.takeIf { it.isHeld }?.release()
        wifiLock = null
        wakeLock?.takeIf { it.isHeld }?.release()
        wakeLock = null
    }

    private fun buildNotification(pcName: String, microphoneActive: Boolean): Notification {
        val manager = getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        val channel = NotificationChannel(
            CHANNEL_ID,
            "${AppConfig.appName} Connection",
            NotificationManager.IMPORTANCE_LOW
        )
        manager.createNotificationChannel(channel)
        return Notification.Builder(this, CHANNEL_ID)
            .setContentTitle("${AppConfig.appName} connected")
            .setContentText(if (microphoneActive) "Using phone microphone on $pcName" else "Connected to $pcName")
            .setSmallIcon(R.drawable.app_icon)
            .setOngoing(true)
            .build()
    }

    companion object {
        private const val CHANNEL_ID = "wirewave_connection"
        private const val NOTIFICATION_ID = 2001
        private const val ACTION_START = "com.wirewave.action.START_LINK"
        private const val ACTION_STOP = "com.wirewave.action.STOP_LINK"
        private const val EXTRA_PC_NAME = "pc_name"
        private const val EXTRA_MICROPHONE_ACTIVE = "microphone_active"

        fun start(context: Context, pcName: String, microphoneActive: Boolean = false) {
            val intent = Intent(context, WireWaveConnectionService::class.java).apply {
                action = ACTION_START
                putExtra(EXTRA_PC_NAME, pcName)
                putExtra(EXTRA_MICROPHONE_ACTIVE, microphoneActive)
            }
            context.startForegroundService(intent)
        }

        fun stop(context: Context) {
            val intent = Intent(context, WireWaveConnectionService::class.java).apply {
                action = ACTION_STOP
            }
            context.startService(intent)
        }
    }
}
