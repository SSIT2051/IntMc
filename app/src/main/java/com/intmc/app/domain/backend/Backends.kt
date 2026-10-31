package com.intmc.app.domain.backend

import com.intmc.app.domain.model.BackendType
import com.intmc.app.domain.model.Server
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

class PumpkinBackend : ServerBackend() {

    override val type: BackendType = BackendType.Pumpkin
    override val displayName: String = "PumpkinMC"
    override val supportedVersions: List<String> = listOf("1.21.2", "1.21.1", "1.21")
    override val defaultVersion: String = "1.21.2"
    override val defaultArgs: List<String> = listOf("nogui")

    override suspend fun prepare(server: Server): Boolean {
        return withContext(Dispatchers.Default) {
            // Check if pumpkin binary exists, download if needed
            true
        }
    }

    override suspend fun start(server: Server): Boolean {
        return withContext(Dispatchers.Default) {
            setState(ServerState.Starting("Launching PumpkinMC..."))
            // ProcessBuilder would go here
            setState(ServerState.Running())
            true
        }
    }

    override suspend fun stop(server: Server): Boolean {
        return withContext(Dispatchers.Default) {
            setState(ServerState.Stopped)
            true
        }
    }

    override suspend fun isRunning(server: Server): Boolean {
        return withContext(Dispatchers.Default) {
            false
        }
    }

    override suspend fun getLogs(server: Server, lines: Int): List<String> {
        return withContext(Dispatchers.Default) {
            emptyList()
        }
    }

    override suspend fun sendCommand(server: Server, command: String): Boolean {
        return withContext(Dispatchers.Default) {
            false
        }
    }
}

class SteelBackend : ServerBackend() {

    override val type: BackendType = BackendType.Steel
    override val displayName: String = "SteelMC"
    override val supportedVersions: List<String> = listOf("1.20.1", "1.18.2", "1.16.5", "1.12.2")
    override val defaultVersion: String = "1.20.1"
    override val defaultArgs: List<String> = listOf("nogui")

    override suspend fun prepare(server: Server): Boolean = true

    override suspend fun start(server: Server): Boolean {
        return withContext(Dispatchers.Default) {
            setState(ServerState.Starting("Launching SteelMC..."))
            setState(ServerState.Running())
            true
        }
    }

    override suspend fun stop(server: Server): Boolean {
        return withContext(Dispatchers.Default) {
            setState(ServerState.Stopped)
            true
        }
    }

    override suspend fun isRunning(server: Server): Boolean = false

    override suspend fun getLogs(server: Server, lines: Int): List<String> =
        emptyList()

    override suspend fun sendCommand(server: Server, command: String): Boolean = false
}

class RosewaterBackend : ServerBackend() {

    override val type: BackendType = BackendType.Rosewater
    override val displayName: String = "Rosewater"
    override val supportedVersions: List<String> = listOf("1.21.1", "1.21")
    override val defaultVersion: String = "1.21.1"
    override val defaultArgs: List<String> = emptyList()

    override suspend fun prepare(server: Server): Boolean = true

    override suspend fun start(server: Server): Boolean {
        return withContext(Dispatchers.Default) {
            setState(ServerState.Starting("Launching Rosewater..."))
            setState(ServerState.Running())
            true
        }
    }

    override suspend fun stop(server: Server): Boolean {
        return withContext(Dispatchers.Default) {
            setState(ServerState.Stopped)
            true
        }
    }

    override suspend fun isRunning(server: Server): Boolean = false

    override suspend fun getLogs(server: Server, lines: Int): List<String> =
        emptyList()

    override suspend fun sendCommand(server: Server, command: String): Boolean = false
}

class GlowstoneBackend : ServerBackend() {

    override val type: BackendType = BackendType.Glowstone
    override val displayName: String = "Glowstone-rs"
    override val supportedVersions: List<String> = listOf("1.18.2", "1.16.5", "1.12.2")
    override val defaultVersion: String = "1.18.2"
    override val defaultArgs: List<String> = emptyList()

    override suspend fun prepare(server: Server): Boolean = true

    override suspend fun start(server: Server): Boolean {
        return withContext(Dispatchers.Default) {
            setState(ServerState.Starting("Launching Glowstone..."))
            setState(ServerState.Running())
            true
        }
    }

    override suspend fun stop(server: Server): Boolean {
        return withContext(Dispatchers.Default) {
            setState(ServerState.Stopped)
            true
        }
    }

    override suspend fun isRunning(server: Server): Boolean = false

    override suspend fun getLogs(server: Server, lines: Int): List<String> =
        emptyList()

    override suspend fun sendCommand(server: Server, command: String): Boolean = false
}

object BackendRegistry {

    val all: List<ServerBackend> = listOf(
        PumpkinBackend(),
        SteelBackend(),
        RosewaterBackend(),
        GlowstoneBackend()
    )

    fun getByType(type: BackendType): ServerBackend =
        all.first { it.type == type }

    fun getByName(name: String): ServerBackend =
        all.first { it.displayName.equals(name, ignoreCase = true) }
}
