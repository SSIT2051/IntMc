package com.intmc.app.ui.screens

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.intmc.app.domain.model.Server
import com.intmc.app.ui.MainViewModel
import com.intmc.app.ui.theme.IntmcAppTheme
import com.intmc.app.di.AppModuleLocal

@Composable
fun ServerDetailScreen(
    server: Server,
    viewModel: MainViewModel,
    onBackClick: () -> Unit,
    modifier: Modifier = Modifier
) {
    val backend = viewModel.getBackendForServer(server)

    Column(modifier = modifier.fillMaxSize().padding(16.dp)) {
        TopAppBar(
            title = { Text(server.name) },
            navigationIcon = {
                IconButton(onClick = onBackClick) {
                    Icon(androidx.compose.material3.Icons.Default.ArrowBack, contentDescription = "Back")
                }
            }
        )

        Spacer(modifier = Modifier.height(16.dp))

        // Status card
        Card(
            modifier = Modifier.fillMaxWidth(),
            elevation = 2.dp
        ) {
            Column(modifier = Modifier.padding(16.dp)) {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text(
                        text = "Status",
                        style = MaterialTheme.typography.titleMedium
                    )
                    Spacer(modifier = Modifier.width(8.dp))
                    val statusColor = if (server.isRunning)
                        MaterialTheme.colorScheme.successContainer
                    else
                        MaterialTheme.colorScheme.errorContainer
                    Box(
                        modifier = Modifier.size(12.dp),
                        contentAlignment = Alignment.Center
                    ) {
                        Circle(
                            color = statusColor,
                            modifier = Modifier.size(12.dp)
                        )
                    }
                    Text(
                        text = if (server.isRunning) "Running" else "Stopped",
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.primary
                    )
                }

                Spacer(modifier = Modifier.height(12.dp))

                Row(
                    horizontalArrangement = androidx.compose.foundation.layout.Arrangement.spaceBetween,
                    modifier = Modifier.fillMaxWidth()
                ) {
                    Text(
                        text = "Backend: ${server.backend}",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                    Text(
                        text = "Port: ${server.port}",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                }

                Spacer(modifier = Modifier.height(12.dp))

                if (server.tunnelUrl.isNotEmpty()) {
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        Text(
                            text = "Tunnel:",
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                        Spacer(modifier = Modifier.width(8.dp))
                        Text(
                            text = server.tunnelUrl,
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.primary,
                            maxLines = 1
                        )
                    }
                }

                Spacer(modifier = Modifier.height(16.dp))

                Row(
                    horizontalArrangement = androidx.compose.foundation.layout.Arrangement.spaceBetween,
                    modifier = Modifier.fillMaxWidth()
                ) {
                    if (server.isRunning) {
                        Button(
                            onClick = { viewModel.stopServer(server) },
                            colors = ButtonDefaults.outlinedButtonColors()
                        ) {
                            Text("Stop Server")
                        }
                        Button(
                            onClick = { /* open console */ }
                        ) {
                            Text("Console")
                        }
                    } else {
                        Button(
                            onClick = { viewModel.startServer(server) }
                        ) {
                            Text("Start Server")
                        }
                    }
                }

                Spacer(modifier = Modifier.height(12.dp))

                Button(
                    onClick = { viewModel.createTunnel(server, server.port) },
                    colors = ButtonDefaults.tonalButtonColors()
                ) {
                    Text("Create Playit Tunnel")
                }
            }
        }

        Spacer(modifier = Modifier.height(16.dp))

        // Logs section
        Text(
            text = "Server Logs",
            style = MaterialTheme.typography.titleMedium,
            modifier = Modifier.fillMaxWidth()
        )

        Spacer(modifier = Modifier.height(8.dp))

        LogViewer(
            logs = emptyList(),
            modifier = Modifier.fillMaxSize().weight(1f)
        )
    }
}

@Composable
fun LogViewer(
    logs: List<String>,
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier
    ) {
        if (logs.isEmpty()) {
            Text(
                text = "No logs available",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
                modifier = Modifier.align(Alignment.Center)
            )
        } else {
            LazyColumn {
                items(logs) { log ->
                    Text(
                        text = log,
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurface,
                        modifier = Modifier.fillMaxWidth()
                    )
                }
            }
        }
    }
}
