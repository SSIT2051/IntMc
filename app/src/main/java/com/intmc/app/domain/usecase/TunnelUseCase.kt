package com.intmc.app.domain.usecase

import android.content.Context
import com.intmc.app.domain.model.Server
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

class TunnelUseCase(private val context: Context) {

    private const val PLAYIT_CLI = "playit"

    suspend fun createTunnel(server: Server, port: Int): String? {
        return withContext(Dispatchers.Default) {
            try {
                val process = java.lang.ProcessBuilder(PLAYIT_CLI, "tunnel", "tcp", ":$port")
                    .redirectErrorStream(true)
                    .start()

                // Read output to find tunnel URL
                val tunnelUrl = readTunnelUrl(process)
                tunnelUrl
            } catch (e: Exception) {
                null
            }
        }
    }

    private fun readTunnelUrl(process: Process): String? {
        val output = process.inputStream.bufferedReader().readText()
        // Parse playit output for tcp://domain.playit.gg:port
        val pattern = "tcp://[\\w.-]+\\.playit\\.gg:\\d+".toRegex()
        return pattern.find(output)?.value
    }

    suspend fun stopTunnel() {
        withContext(Dispatchers.Default) {
            try {
                java.lang.ProcessBuilder("pkill", "-f", "playit").start()
            } catch (e: Exception) {
                // Ignore
            }
        }
    }
}
