package com.intmc.app.domain.usecase

import com.intmc.app.domain.model.Server
import com.intmc.app.domain.backend.ServerBackend
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

class ServerUseCase {

    suspend fun prepareServer(backend: ServerBackend, server: Server): Boolean {
        return withContext(Dispatchers.Default) {
            backend.prepare(server)
        }
    }

    suspend fun startServer(backend: ServerBackend, server: Server): Boolean {
        return withContext(Dispatchers.Default) {
            backend.start(server)
        }
    }

    suspend fun stopServer(backend: ServerBackend, server: Server): Boolean {
        return withContext(Dispatchers.Default) {
            backend.stop(server)
        }
    }

    suspend fun isServerRunning(backend: ServerBackend, server: Server): Boolean {
        return withContext(Dispatchers.Default) {
            backend.isRunning(server)
        }
    }
}
