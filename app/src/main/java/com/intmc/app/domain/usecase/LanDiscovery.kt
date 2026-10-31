package com.intmc.app.domain.usecase

import android.content.Context
import java.io.IOException
import java.net.DatagramPacket
import java.net.DatagramSocket
import java.net.InetAddress
import kotlin.concurrent.thread

class LanDiscovery {

    private const val BROADCAST_PORT = 19132
    private const val SCAN_TIMEOUT_MS = 3000

    data class LanResult(
        val name: String,
        val ip: String,
        val port: Int,
        val version: String,
        val motd: String,
        val playersOnline: Int,
        val maxPlayers: Int
    )

    suspend fun scan(): List<LanResult> {
        return withContext(Dispatchers.Default) {
            val found = mutableListOf<LanResult>()
            val socket = DatagramSocket()
            socket.broadcast = true
            socket.soTimeout = SCAN_TIMEOUT_MS

            try {
                // Send discovery broadcast
                val msg = "MC|Ping|Discovery".toByteArray()
                val packet = DatagramPacket(
                    msg, msg.size,
                    InetAddress.getByName("255.255.255.255"),
                    BROADCAST_PORT
                )
                socket.send(packet)

                // Listen for responses
                val buffer = ByteArray(1024)
                val endTime = System.currentTimeMillis() + SCAN_TIMEOUT_MS
                while (System.currentTimeMillis() < endTime) {
                    try {
                        val response = DatagramPacket(buffer, buffer.size)
                        socket.receive(response)
                        val data = String(response.data, 0, response.length)
                        val parsed = parseLanResponse(data, response.address.hostAddress)
                        if (parsed != null) {
                            found.add(parsed)
                        }
                    } catch (e: java.net.SocketTimeoutException) {
                        break
                    }
                }
            } catch (e: IOException) {
                // Network not available
            } finally {
                socket.close()
            }

            found
        }
    }

    private fun parseLanResponse(data: String, ip: String): LanResult? {
        // Format: MOTD;Players;MaxPlayers;Version;ServerID
        val parts = data.split(";")
        if (parts.size >= 5) {
            return LanResult(
                name = parts[4],
                ip = ip,
                port = 19132,
                version = parts[3],
                motd = parts[0],
                playersOnline = parts[1].toIntOrNull() ?: 0,
                maxPlayers = parts[2].toIntOrNull() ?: 0
            )
        }
        return null
    }
}
