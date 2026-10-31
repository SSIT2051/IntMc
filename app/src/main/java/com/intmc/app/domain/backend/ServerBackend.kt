package com.intmc.app.domain.backend

import com.intmc.app.domain.model.BackendType
import com.intmc.app.domain.model.Server
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow

sealed class ServerState {
    object Stopped : ServerState()
    data class Starting(val message: String = "") : ServerState()
    data class Running(val tunnelUrl: String? = null) : ServerState()
    data class Stopping(val message: String = "") : ServerState()
    data class Error(val message: String) : ServerState()
}

abstract class ServerBackend {

    abstract val type: BackendType
    abstract val displayName: String
    abstract val supportedVersions: List<String>
    abstract val defaultVersion: String
    abstract val defaultArgs: List<String>

    protected val _state = MutableStateFlow(ServerState.Stopped)
    val state: StateFlow<ServerState> = _state

    abstract suspend fun prepare(server: Server): Boolean
    abstract suspend fun start(server: Server): Boolean
    abstract suspend fun stop(server: Server): Boolean
    abstract suspend fun isRunning(server: Server): Boolean
    abstract suspend fun getLogs(server: Server, lines: Int): List<String>
    abstract suspend fun sendCommand(server: Server, command: String): Boolean

    protected fun setState(state: ServerState) {
        _state.value = state
    }
}
