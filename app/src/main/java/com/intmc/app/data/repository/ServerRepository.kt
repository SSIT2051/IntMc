package com.intmc.app.data.repository

import android.content.Context
import com.intmc.app.data.local.AppDatabase
import com.intmc.app.data.local.dao.PluginDao
import com.intmc.app.data.local.dao.ServerDao
import com.intmc.app.data.local.entity.PluginEntity
import com.intmc.app.data.local.entity.ServerEntity
import com.intmc.app.domain.model.LanServer
import com.intmc.app.domain.model.Plugin
import com.intmc.app.domain.model.Server
import com.intmc.app.domain.usecase.LanDiscovery
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

class ServerRepository private constructor(
    private val serverDao: ServerDao,
    private val pluginDao: PluginDao,
    private val lanDiscovery: LanDiscovery
) {

    fun getServers() = serverDao.getAllServers()

    suspend fun getServerById(id: String): ServerEntity? =
        serverDao.getServerById(id)

    suspend fun createServer(server: ServerEntity): Long =
        serverDao.insertServer(server)

    suspend fun updateServer(server: ServerEntity) =
        serverDao.updateServer(server)

    suspend fun deleteServer(id: String) {
        val entity = serverDao.getServerById(id)
        if (entity != null) {
            pluginDao.deletePluginsByServer(id)
            serverDao.deleteServerById(id)
        }
    }

    suspend fun setRunningStatus(id: String, running: Boolean) =
        serverDao.updateRunningStatus(id, running)

    suspend fun setLastStarted(id: String, ts: Long) =
        serverDao.updateLastStarted(id, ts)

    suspend fun setTunnelUrl(id: String, url: String) =
        serverDao.updateTunnelUrl(id, url)

    // Plugins
    fun getPluginsForServer(serverId: String) = pluginDao.getPluginsByServer(serverId)

    suspend fun addPlugin(plugin: PluginEntity): Long =
        pluginDao.insertPlugin(plugin)

    suspend fun updatePlugin(plugin: PluginEntity) =
        pluginDao.updatePlugin(plugin)

    suspend fun deletePlugin(plugin: PluginEntity) =
        pluginDao.deletePlugin(plugin)

    // LAN
    suspend fun discoverLan(): List<LanServer> =
        lanDiscovery.scan()

    companion object {
        @Volatile private var INSTANCE: ServerRepository? = null

        fun getInstance(context: Context): ServerRepository {
            return INSTANCE ?: synchronized(this) {
                val db = AppDatabase.getDatabase(context)
                INSTANCE = ServerRepository(
                    serverDao = db.serverDao(),
                    pluginDao = db.pluginDao(),
                    lanDiscovery = LanDiscovery()
                )
                INSTANCE!!
            }
        }
    }
}

// Domain model mappings
fun ServerEntity.toDomain(): Server =
    Server(
        id = id,
        name = name,
        backend = backend,
        version = version,
        port = port,
        ip = ip,
        isRunning = isRunning,
        path = path,
        autoTunnel = autoTunnel,
        createdAt = createdAt,
        lastStarted = lastStarted,
        memoryMb = memoryMb,
        tunnelUrl = tunnelUrl
    )

fun Server.toEntity(): ServerEntity =
    ServerEntity(
        id = id,
        name = name,
        backend = backend,
        version = version,
        port = port,
        ip = ip,
        isRunning = isRunning,
        path = path,
        autoTunnel = autoTunnel,
        createdAt = createdAt,
        lastStarted = lastStarted,
        memoryMb = memoryMb,
        tunnelUrl = tunnelUrl
    )

fun PluginEntity.toDomain(): Plugin =
    Plugin(
        id = id,
        serverId = serverId,
        name = name,
        version = version,
        source = source,
        format = format,
        enabled = enabled,
        downloadUrl = downloadUrl,
        installedAt = installedAt
    )

fun Plugin.toEntity(): PluginEntity =
    PluginEntity(
        id = id,
        serverId = serverId,
        name = name,
        version = version,
        source = source,
        format = format,
        enabled = enabled,
        downloadUrl = downloadUrl,
        installedAt = installedAt
    )
