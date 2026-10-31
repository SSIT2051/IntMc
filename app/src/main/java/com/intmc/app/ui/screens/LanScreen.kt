package com.intmc.app.ui.screens

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.intmc.app.domain.model.LanServer
import com.intmc.app.domain.usecase.LanDiscovery
import com.intmc.app.ui.MainViewModel
import com.intmc.app.ui.theme.IntmcAppTheme
import com.intmc.app.di.AppModuleLocal
import kotlinx.coroutines.launch

@Composable
fun LanScreen(
    viewModel: MainViewModel,
    onBackClick: () -> Unit,
    modifier: Modifier = Modifier
) {
    val lanDiscovery = LanDiscovery()
    var lanServers by remember { mutableStateOf<List<LanServer>>(emptyList()) }
    var isScanning by remember { mutableStateOf(false) }
    var statusMessage by remember { mutableStateOf("") }

    Column(modifier = modifier.fillMaxSize().padding(16.dp)) {
        TopAppBar(
            title = { Text("LAN Discovery") },
            navigationIcon = {
                IconButton(onClick = onBackClick) {
                    Icon(androidx.compose.material3.Icons.Default.ArrowBack, contentDescription = "Back")
                }
            }
        )

        Spacer(modifier = Modifier.height(16.dp))

        Button(
            onClick = {
                isScanning = true
                statusMessage = "Scanning for LAN servers..."
                // Run scan
                kotlinx.coroutines.CoroutineScope(kotlinx.coroutines.Dispatchers.Default).launch {
                    val results = lanDiscovery.scan()
                    lanServers = results
                    isScanning = false
                    statusMessage = if (results.isEmpty()) "No LAN servers found" else "Found ${results.size} server(s)"
                }
            }
        ) {
            if (isScanning) {
                CircularProgressIndicator(modifier = Modifier.size(20.dp))
            } else {
                Icon(androidx.compose.material3.Icons.Default.Wifi, contentDescription = "Scan")
                Text("Scan LAN")
            }
        }

        Spacer(modifier = Modifier.height(16.dp))

        if (statusMessage.isNotEmpty()) {
            Text(
                text = statusMessage,
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.primary,
                modifier = Modifier.padding(vertical = 8.dp)
            )
        }

        if (lanServers.isEmpty()) {
            Text(
                text = "No LAN servers discovered",
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
                modifier = Modifier.align(Alignment.Center)
            )
        } else {
            LazyColumn(
                modifier = Modifier.fillMaxSize()
            ) {
                items(lanServers, key = { it.ip }) { server ->
                    Card(
                        modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp),
                        elevation = 1.dp
                    ) {
                        Column(modifier = Modifier.padding(16.dp)) {
                            Row(
                                modifier = Modifier.fillMaxWidth(),
                                verticalAlignment = Alignment.CenterVertically
                            ) {
                                Text(
                                    text = server.name,
                                    style = MaterialTheme.typography.titleLarge,
                                    modifier = Modifier.weight(1f)
                                )
                                Chip(
                                    label = { Text(text = "${server.playersOnline}/${server.maxPlayers}") },
                                    colors = ChipDefaults.colors(
                                        containerColor = MaterialTheme.colorScheme.secondaryContainer
                                    )
                                )
                            }

                            Spacer(modifier = Modifier.height(8.dp))

                            Row(
                                modifier = Modifier.fillMaxWidth(),
                                verticalAlignment = Alignment.CenterVertically
                            ) {
                                Text(
                                    text = "${server.ip}:${server.port}  •  ${server.version}",
                                    style = MaterialTheme.typography.bodyMedium
                                )
                                Spacer(modifier = Modifier.width(8.dp))
                                Text(
                                    text = server.motd,
                                    style = MaterialTheme.typography.bodySmall,
                                    color = MaterialTheme.colorScheme.onSurfaceVariant
                                )
                            }

                            Spacer(modifier = Modifier.height(12.dp))

                            Button(
                                onClick = { /* join server */ }
                            ) {
                                Text("Join")
                            }
                        }
                    }
                }
            }
        }
    }
}
