package io.nativeblocks.devkit.util

import android.content.Context
import android.hardware.Sensor
import android.hardware.SensorEvent
import android.hardware.SensorEventListener
import android.hardware.SensorManager
import kotlin.math.sqrt

internal interface ShakeDetectorCallback {
    fun onShakeDetected()
}

internal class ShakeDetector(
    context: Context,
    private val threshold: Float = 12.0f,
    private val timeWindowMs: Long = 500L,
    private val shakeCount: Int = 3,
    private val callback: ShakeDetectorCallback
) : SensorEventListener {

    private val sensorManager: SensorManager =
        context.getSystemService(Context.SENSOR_SERVICE) as SensorManager
    private val accelerometer: Sensor? =
        sensorManager.getDefaultSensor(Sensor.TYPE_ACCELEROMETER)

    private var lastShakeTime: Long = 0
    private var shakeCounter: Int = 0
    private var lastX: Float = 0f
    private var lastY: Float = 0f
    private var lastZ: Float = 0f
    private var isFirstReading: Boolean = true

    fun start() {
        accelerometer?.let {
            sensorManager.registerListener(
                this,
                it,
                SensorManager.SENSOR_DELAY_UI
            )
        }
    }

    fun stop() {
        sensorManager.unregisterListener(this)
    }

    override fun onSensorChanged(event: SensorEvent?) {
        event ?: return
        if (event.sensor.type != Sensor.TYPE_ACCELEROMETER) return

        val x = event.values[0]
        val y = event.values[1]
        val z = event.values[2]

        if (isFirstReading) {
            lastX = x
            lastY = y
            lastZ = z
            isFirstReading = false
            return
        }

        val deltaX = x - lastX
        val deltaY = y - lastY
        val deltaZ = z - lastZ

        lastX = x
        lastY = y
        lastZ = z

        val acceleration = sqrt(deltaX * deltaX + deltaY * deltaY + deltaZ * deltaZ)

        if (acceleration > threshold) {
            val currentTime = System.currentTimeMillis()

            if (currentTime - lastShakeTime > timeWindowMs) {
                shakeCounter = 0
            }

            lastShakeTime = currentTime
            shakeCounter++

            if (shakeCounter >= shakeCount) {
                shakeCounter = 0
                callback.onShakeDetected()
            }
        }
    }

    override fun onAccuracyChanged(sensor: Sensor?, accuracy: Int) {
        // Not needed
    }
}
