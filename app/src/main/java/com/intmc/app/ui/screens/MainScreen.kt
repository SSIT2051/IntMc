package com.intmc.app.ui.screens

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.intmc.app.ui.MainViewModel
import com.intmc.app.ui.theme.IntmcAppTheme
import com.intmc.app.di.AppModuleLocal

@Composable
fun MainScreen(navController: androidx.navigation.NavController) {
    val appModule = AppModuleLocal.current
    val context = appModule.context
    val viewModel: MainViewModel = viewModel(
        factory = MainViewModelFactory(context)
    )

    IntmcAppTheme {
        Scaffold(
            topBar = {
                TopAppBar(
                    title = { Text(text = "intmc") },
                    colors = TopAppBarDefaults.mediumTopAppBarColors(
                        containerColor = MaterialTheme.colorScheme.primary
                    )
                )
            },
            floatingActionButton = {
                FloatingActionButton(
                    onClick = { /* navController.navigate(R.id.createServerRoute) */ },
                    content = { Icon(Icons.Default.Add, contentDescription = "Create Server") }
                )
            }
        ) { padding ->
            ServerListScreen(
                viewModel = viewModel,
                modifier = Modifier.padding(padding)
            )
        }
    }
}

@Composable
fun ServerListScreen(
    viewModel: MainViewModel,
    modifier: Modifier = Modifier
) {
    val servers by viewModel.servers.collectAsState()
    val statusMessage by viewModel.statusMessage.collectAsState()
    val errorMessage by viewModel.errorMessage.collectAsState()

    Column(modifier = modifier) {
        if (statusMessage.isNotEmpty()) {
            Text(
                text = statusMessage,
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.primary,
                modifier = Modifier.padding(16.dp)
            )
        }

        if (errorMessage.isNotEmpty()) {
            Text(
                text = errorMessage,
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.error,
                modifier = Modifier.padding(16.dp)
            )
        }

        if (servers.isEmpty()) {
            Box(
                modifier = Modifier.fillMaxSize(),
                contentAlignment = Alignment.Center
            ) {
                Column(horizontalAlignment = Alignment.CenterHorizontally) {
                    Icon(
                        imageVector = Icons.Default.Cloud,
                        contentDescription = "No servers",
                        modifier = Modifier.size(64.dp)
                    )
                    Text(
                        text = "No servers yet",
                        style = MaterialTheme.typography.titleMedium
                    )
                    Text(
                        text = "Tap + to create your first Minecraft server",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                }
            }
        } else {
            LazyColumn(
                modifier = Modifier.fillMaxSize()
            ) {
                items(servers, key = { it.id }) { server ->
                    ServerCard(
                        server = server,
                        onStartClick = { viewModel.startServer(server) },
                        onStopClick = { viewModel.stopServer(server) },
                        onDeleteClick = { viewModel.deleteServer(server) },
                        onDetailClick = { /* navigate to detail */ }
                    )
                    Divider()
                }
            }
        }
    }
}

@Composable
fun ServerCard(
    server: com.intmc.app.domain.model.Server,
    onStartClick: () -> Unit,
    onStopClick: () -> Unit,
    onDeleteClick: () -> Unit,
    onDetailClick: () -> Unit
) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 16.dp, vertical = 8.dp),
        elevation = 2.dp
    ) {
        Column(modifier = Modifier.padding(16.dp)) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                verticalAlignment = androidx.compose.ui.Alignment.CenterVertically
            ) {
                Text(
                    text = server.name,
                    style = MaterialTheme.typography.titleLarge,
                    modifier = Modifier.weight(1f)
                )
                Chip(
                    label = { Text(text = server.backend) },
                    colors = ChipDefaults.colors(
                        containerColor = MaterialTheme.colorScheme.primaryContainer
                    )
                )
            }

            Row(
                modifier = Modifier.fillMaxWidth(),
                verticalAlignment = androidx.compose.ui.Alignment.CenterVertically
            ) {
                Text(
                    text = "Port: ${server.port}  •  Memory: ${server.memoryMb}MB",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }

            Spacer(modifier = Modifier.height(12.dp))

            Row(
                horizontalArrangement = Arrangement.spaceBetween,
                modifier = Modifier.fillMaxWidth()
            ) {
                if (server.isRunning) {
                    Button(onClick = onStopClick) {
                        Text("Stop")
                    }
                    Button(
                        onClick = onDetailClick,
                        colors = ButtonDefaults.outlinedButtonColors()
                    ) {
                        Text("Console")
                    }
                } else {
                    Button(onClick = onStartClick) {
                        Text("Start")
                    }
                }
                OutlinedButton(onClick = onDeleteClick) {
                    Text("Delete")
                }
            }
        }
    }
}

@Composable
fun CreateServerScreen(
    onBackClick: () -> Unit,
    onServerCreated: (com.intmc.app.domain.model.Server) -> Unit
) {
    var name by remember { mutableStateOf("") }
    var backend by remember { mutableStateOf("PumpkinMC") }
    var port by remember { mutableStateOf("25565") }
    var version by remember { mutableStateOf("1.21.2") }
    var memoryMb by remember { mutableStateOf("1024") }

    val backends = listOf("PumpkinMC", "SteelMC", "Rosewater", "Glowstone-rs")

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {
        TopAppBar(
            title = { Text("Create Server") },
            navigationIcon = {
                IconButton(onClick = onBackClick) {
                    Icon(Icons.Default.ArrowBack, contentDescription = "Back")
                }
            }
        )

        Spacer(modifier = Modifier.height(16.dp))

        TextField(
            value = name,
            onValueChange = { name = it },
            label = { Text("Server Name") },
            modifier = Modifier.fillMaxWidth()
        )

        Spacer(modifier = Modifier.height(16.dp))

        DropdownMenu(
            expanded = true,
            onDismissRequest = {},
            modifier = Modifier.fillMaxWidth()
        ) {
            backends.forEach { b ->
                DropdownMenuItem(
                    text = { Text(text = b) },
                    onClick = { backend = b }
                )
            }
        }

        Spacer(modifier = Modifier.height(16.dp))

        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spaceBetween
        ) {
            Column {
                Text(text = "Port", style = MaterialTheme.typography.labelSmall)
                TextField(
                    value = port,
                    onValueChange = { port = it },
                    modifier = Modifier.width(100.dp),
                    keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number)
                )
            }
            Column {
                Text(text = "Memory (MB)", style = MaterialTheme.typography.labelSmall)
                TextField(
                    value = memoryMb,
                    onValueChange = { memoryMb = it },
                    modifier = Modifier.width(100.dp),
                    keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number)
                )
            }
        }

        Spacer(modifier = Modifier.height(16.dp))

        TextField(
            value = version,
            onValueChange = { version = it },
            label = { Text("Version") },
            modifier = Modifier.fillMaxWidth()
        )

        Spacer(modifier = Modifier.height(24.dp))

        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spaceBetween
        ) {
            Button(onClick = onBackClick) {
                Text("Cancel")
            }
            Button(
                onClick = {
                    val portNum = port.toIntOrNull() ?: 25565
                    val memNum = memoryMb.toIntOrNull() ?: 1024
                    val server = com.intmc.app.domain.model.Server(
                        id = java.util.UUID.randomUUID().toString(),
                        name = name,
                        backend = backend,
                        port = portNum,
                        version = version,
                        memoryMb = memNum
                    )
                    onServerCreated(server)
                }
            ) {
                Text("Create")
            }
        }
    }
}
