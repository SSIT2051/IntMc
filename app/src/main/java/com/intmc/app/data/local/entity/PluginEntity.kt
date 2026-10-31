package com.intmc.app.data.local.entity

import androidx.room.Entity
import androidx.room.ForeignKey
import androidx.room.PrimaryKey
import androidx.room.ColumnInfo
import androidx.room.Index

@Entity(
    tableName = "plugins",
    foreignKeys = [
        ForeignKey(
            entity = ServerEntity::class,
            parentColumns = ["id"],
            childColumns = ["server_id"],
            onDelete = ForeignKey.CASCADE
        )
    ],
    indices = [Index("server_id")]
)
data class PluginEntity(
    @PrimaryKey
    val id: String = java.util.UUID.randomUUID().toString(),
    @ColumnInfo(name = "server_id")
    val serverId: String,
    @ColumnInfo(name = "name")
    val name: String,
    @ColumnInfo(name = "version")
    val version: String = "",
    @ColumnInfo(name = "source")
    val source: String = "",
    @ColumnInfo(name = "format")
    val format: String = "jar",
    @ColumnInfo(name = "enabled")
    val enabled: Boolean = false,
    @ColumnInfo(name = "download_url")
    val downloadUrl: String = "",
    @ColumnInfo(name = "installed_at")
    val installedAt: Long = System.currentTimeMillis()
)
