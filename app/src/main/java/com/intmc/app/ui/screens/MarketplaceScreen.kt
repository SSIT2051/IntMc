package com.intmc.app.ui.screens

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.intmc.app.ui.MainViewModel
import com.intmc.app.ui.theme.IntmcAppTheme
import com.intmc.app.di.AppModuleLocal

@Composable
fun MarketplaceScreen(
    viewModel: MainViewModel,
    onBackClick: () -> Unit,
    modifier: Modifier = Modifier
) {
    Column(modifier = modifier.fillMaxSize().padding(16.dp)) {
        TopAppBar(
            title = { Text("Plugin Marketplace") },
            navigationIcon = {
                IconButton(onClick = onBackClick) {
                    Icon(androidx.compose.material3.Icons.Default.ArrowBack, contentDescription = "Back")
                }
            }
        )

        Spacer(modifier = Modifier.height(16.dp))

        TextField(
            value = "",
            onValueChange = {},
            label = { Text("Search plugins") },
            modifier = Modifier.fillMaxWidth()
        )

        Spacer(modifier = Modifier.height(16.dp))

        Button(
            onClick = { /* refresh marketplace */ }
        ) {
            Text("Refresh")
        }

        Spacer(modifier = Modifier.height(16.dp))

        Text(
            text = "Available Plugins",
            style = MaterialTheme.typography.titleMedium
        )

        Spacer(modifier = Modifier.height(8.dp))

        // Placeholder - plugin marketplace would list available plugins
        Text(
            text = "Browse and install plugins for your servers.",
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
    }
}
