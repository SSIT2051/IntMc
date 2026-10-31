package com.intmc.app.data.local.entity

import androidx.room.Entity
import androidx.room.PrimaryKey
import androidx.room.ColumnInfo
import java.util.UUID

@Entity(tableName = "servers")
data class ServerEntity(
    @PrimaryKey
    val id: String = UUID.randomUUID().toString(),
    @ColumnInfo(name = "name")
    val name: String,
    @ColumnInfo(name = "backend")
    val backend: String,
    @ColumnInfo(name = "version")
    val version: String = "",
    @ColumnInfo(name = "port")
    val port: Int = 25565,
    @ColumnInfo(name = "ip")
    val ip: String = "",
    @ColumnInfo(name = "is_running")
    val isRunning: Boolean = false,
    @ColumnInfo(name = "path")
    val path: String = "",
    @ColumnInfo(name = "auto_tunnel")
    val autoTunnel: Boolean = false,
    @ColumnInfo(name = "created_at")
    val createdAt: Long = System.currentTimeMillis(),
    @ColumnInfo(name = "last_started")
    val lastStarted: Long = 0,
    @ColumnInfo(name = "memory_mb")
    val memoryMb: Int = 1024,
    @ColumnInfo(name = "tunnel_url")
    val tunnelUrl: String = ""
)
