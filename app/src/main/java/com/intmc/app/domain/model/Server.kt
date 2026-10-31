package com.intmc.app.domain.model

data class Server(
    val id: String,
    val name: String,
    val backend: String,
    val version: String = "",
    val port: Int = 25565,
    val ip: String = "",
    val isRunning: Boolean = false,
    val path: String = "",
    val autoTunnel: Boolean = false,
    val createdAt: Long = System.currentTimeMillis(),
    val lastStarted: Long = 0,
    val memoryMb: Int = 1024,
    val tunnelUrl: String = ""
)

data class Plugin(
    val id: String,
    val serverId: String,
    val name: String,
    val version: String = "",
    val source: String = "",
    val format: String = "jar",
    val enabled: Boolean = false,
    val downloadUrl: String = "",
    val installedAt: Long = System.currentTimeMillis()
)

data class LanServer(
    val name: String,
    val ip: String,
    val port: Int,
    val version: String,
    val motd: String,
    val playersOnline: Int,
    val maxPlayers: Int
)

enum class BackendType {
    Pumpkin,
    Steel,
    Rosewater,
    Glowstone
}

data class ServerConfig(
    val name: String,
    val backend: BackendType,
    val port: Int,
    val version: String,
    val memoryMb: Int
)
