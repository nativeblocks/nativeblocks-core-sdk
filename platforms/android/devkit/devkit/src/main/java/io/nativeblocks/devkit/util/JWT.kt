package io.nativeblocks.devkit.util

import android.util.Base64
import org.json.JSONObject

internal object JWT {
    fun decodedBody(encoded: String): JSONObject? {
        try {
            val split = encoded.split("\\.".toRegex()).dropLastWhile { it.isEmpty() }.toTypedArray()
            DevKitLogger.d("JWT_DECODED", "Body: " + getJson(split[1]))
            return JSONObject(getJson(split[1]))
        } catch (e: Exception) {
            return null
        }
    }

    private fun getJson(strEncoded: String): String {
        val decodedBytes: ByteArray = Base64.decode(strEncoded, Base64.URL_SAFE)
        return String(decodedBytes, charset("UTF-8"))
    }
}