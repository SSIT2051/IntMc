package com.intmc.app.ui

import android.content.Context
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.intmc.app.data.local.entity.ServerEntity
import com.intmc.app.data.repository.ServerRepository
import com.intmc.app.domain.backend.BackendRegistry
import com.intmc.app.domain.backend.ServerBackend
import com.intmc.app.domain.backend.ServerState
import com.intmc.app.domain.model.Server
import com.intmc.app.domain.usecase.ServerUseCase
import com.intmc.app.domain.usecase.TunnelUseCase
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch

class MainViewModel(private val context: Context) : ViewModel() {

    private val repository = ServerRepository.getInstance(context)
    private val backendRegistry = BackendRegistry
    private val serverUseCase = ServerUseCase()
    private val tunnelUseCase = TunnelUseCase(context)

    private val _servers = MutableStateFlow<List<Server>>(emptyList())
    val servers: StateFlow<List<Server>> = _servers.stateIn(
        viewModelScope,
        androidx.lifecycle.SharingStarted.WhileSubscribed(5000),
        emptyList()
    )

    private val _statusMessage = MutableStateFlow<String>("")
    val statusMessage: StateFlow<String> = _statusMessage.stateIn(
        viewModelScope,
        SharingStarted.WhileSubscribed(5000),
        ""
    )

    private val _errorMessage = MutableStateFlow<String>("")
    val errorMessage: StateFlow<String> = _errorMessage.stateIn(
        viewModelScope,
        SharingStarted.WhileSubscribed(5000),
        ""
    )

    init {
        loadServers()
    }

    fun loadServers() {
        viewModelScope.launch {
            try {
                val entities = repository.getServers().first()
                _servers.value = entities.map { it.toDomain() }
                _statusMessage.value = "Loaded ${entities.size} servers"
            } catch (e: Exception) {
                _errorMessage.value = e.message ?: "Failed to load servers"
            }
        }
    }

    fun createServer(name: String, backendName: String, port: Int, version: String, memoryMb: Int) {
        viewModelScope.launch {
            try {
                val backend = backendRegistry.getByName(backendName)
                val server = Server(
                    name = name,
                    backend = backend.displayName,
                    port = port,
                    version = version,
                    memoryMb = memoryMb
                )
                val entity = server.toEntity()
                val id = repository.createServer(entity)
                val createdServer = server.copy(id = id.toString())
                _servers.value = _servers.value + createdServer
                _statusMessage.value = "Server '$name' created"

                // Auto-tunnel if enabled
                // tunnel creation would be triggered from detail screen
            } catch (e: Exception) {
                _errorMessage.value = e.message ?: "Failed to create server"
            }
        }
    }

    fun startServer(server: Server) {
        viewModelScope.launch {
            try {
                val backend = backendRegistry.getByName(server.backend)
                val success = serverUseCase.startServer(backend, server)
                if (success) {
                    repository.setRunningStatus(server.id, true)
                    repository.setLastStarted(server.id, System.currentTimeMillis())
                    loadServers()
                    _statusMessage.value = "${server.name} started"
                }
            } catch (e: Exception) {
                _errorMessage.value = e.message ?: "Failed to start server"
            }
        }
    }

    fun stopServer(server: Server) {
        viewModelScope.launch {
            try {
                val backend = backendRegistry.getByName(server.backend)
                val success = serverUseCase.stopServer(backend, server)
                if (success) {
                    repository.setRunningStatus(server.id, false)
                    loadServers()
                    _statusMessage.value = "${server.name} stopped"
                }
            } catch (e: Exception) {
                _errorMessage.value = e.message ?: "Failed to stop server"
            }
        }
    }

    fun deleteServer(server: Server) {
        viewModelScope.launch {
            try {
                repository.deleteServer(server.id)
                _servers.value = _servers.value.filter { it.id != server.id }
                _statusMessage.value = "Server deleted"
            } catch (e: Exception) {
                _errorMessage.value = e.message ?: "Failed to delete server"
            }
        }
    }

    fun getBackendForServer(server: Server): ServerBackend {
        return backendRegistry.getByName(server.backend)
    }

    val backends: List<String> = backendRegistry.all.map { it.displayName }

    fun sendCommand(server: Server, command: String) {
        viewModelScope.launch {
            try {
                val backend = backendRegistry.getByName(server.backend)
                val success = serverUseCase.startServer(backend, server) // placeholder - sendCommand
                if (success) {
                    _statusMessage.value = "Command sent: $command"
                }
            } catch (e: Exception) {
                _errorMessage.value = e.message ?: "Failed to send command"
            }
        }
    }

    fun createTunnel(server: Server, port: Int) {
        viewModelScope.launch {
            try {
                val url = tunnelUseCase.createTunnel(server, port)
                if (url != null) {
                    repository.setTunnelUrl(server.id, url)
                    _statusMessage.value = "Tunnel created: $url"
                } else {
                    _errorMessage.value = "Failed to create tunnel"
                }
            } catch (e: Exception) {
                _errorMessage.value = e.message ?: "Failed to create tunnel"
            }
        }
    }

    fun clearMessages() {
        _statusMessage.value = ""
        _errorMessage.value = ""
    }
}
