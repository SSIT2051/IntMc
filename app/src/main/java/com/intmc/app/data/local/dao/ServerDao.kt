package com.intmc.app.data.local.dao

import androidx.room.*
import com.intmc.app.data.local.entity.PluginEntity
import com.intmc.app.data.local.entity.ServerEntity
import kotlinx.coroutines.flow.Flow

@Dao
interface ServerDao {

    @Query("SELECT * FROM servers ORDER BY created_at DESC")
    fun getAllServers(): Flow<List<ServerEntity>>

    @Query("SELECT * FROM servers WHERE id = :id")
    suspend fun getServerById(id: String): ServerEntity?

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insertServer(server: ServerEntity): Long

    @Update
    suspend fun updateServer(server: ServerEntity)

    @Delete
    suspend fun deleteServer(server: ServerEntity)

    @Query("DELETE FROM servers WHERE id = :id")
    suspend fun deleteServerById(id: String)

    @Query("UPDATE servers SET is_running = :running WHERE id = :id")
    suspend fun updateRunningStatus(id: String, running: Boolean)

    @Query("UPDATE servers SET last_started = :ts WHERE id = :id")
    suspend fun updateLastStarted(id: String, ts: Long)

    @Query("UPDATE servers SET tunnel_url = :url WHERE id = :id")
    suspend fun updateTunnelUrl(id: String, url: String)

    @Query("DELETE FROM servers")
    suspend fun deleteAll()
}

@Dao
interface PluginDao {

    @Query("SELECT * FROM plugins WHERE server_id = :serverId ORDER BY name")
    fun getPluginsByServer(serverId: String): Flow<List<PluginEntity>>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insertPlugin(plugin: PluginEntity): Long

    @Update
    suspend fun updatePlugin(plugin: PluginEntity)

    @Delete
    suspend fun deletePlugin(plugin: PluginEntity)

    @Query("DELETE FROM plugins WHERE server_id = :serverId")
    suspend fun deletePluginsByServer(serverId: String)
}
