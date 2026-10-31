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
fun SettingsScreen(
    viewModel: MainViewModel,
    onBackClick: () -> Unit,
    modifier: Modifier = Modifier
) {
    Column(modifier = modifier.fillMaxSize().padding(16.dp)) {
        TopAppBar(
            title = { Text("Settings") },
            navigationIcon = {
                IconButton(onClick = onBackClick) {
                    Icon(androidx.compose.material3.Icons.Default.ArrowBack, contentDescription = "Back")
                }
            }
        )

        Spacer(modifier = Modifier.height(16.dp))

        SettingsItem(
            title = "Dark Theme",
            subtitle = "Use dark theme for better battery life",
            checked = true,
            onCheckedChange = { }
        )

        SettingsItem(
            title = "Notifications",
            subtitle = "Receive server status notifications",
            checked = false,
            onCheckedChange = { }
        )

        SettingsItem(
            title = "Auto-refresh",
            subtitle = "Automatically refresh server status",
            checked = true,
            onCheckedChange = { }
        )

        Spacer(modifier = Modifier.height(24.dp))

        Text(
            text = "Storage",
            style = MaterialTheme.typography.titleMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )

        Spacer(modifier = Modifier.height(8.dp))

        SettingsItem(
            title = "Server Files Path",
            subtitle = "/data/data/com.intmc.app/files/servers",
            onClick = { }
        )

        SettingsItem(
            title = "Backup Location",
            subtitle = "Auto-backup server worlds",
            onClick = { }
        )

        Spacer(modifier = Modifier.height(24.dp))

        Text(
            text = "About",
            style = MaterialTheme.typography.titleMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )

        Spacer(modifier = Modifier.height(8.dp))

        SettingsItem(
            title = "Version",
            subtitle = "1.0.0",
            onClick = { }
        )
    }
}

@Composable
fun SettingsItem(
    title: String,
    subtitle: String = "",
    checked: Boolean = false,
    onCheckedChange: ((Boolean) -> Unit)? = null,
    onClick: (() -> Unit)? = null
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 8.dp),
        verticalAlignment = androidx.compose.ui.Alignment.CenterVertically
    ) {
        Column(
            modifier = Modifier.weight(1f)
        ) {
            Text(
                text = title,
                style = MaterialTheme.typography.bodyMedium
            )
            if (subtitle.isNotEmpty()) {
                Text(
                    text = subtitle,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
        }

        if (onCheckedChange != null) {
            Checkbox(
                checked = checked,
                onCheckedChange = onCheckedChange,
                colors = CheckboxDefaults.colors(
                    checkedColor = MaterialTheme.colorScheme.primary
                )
            )
        } else if (onClick != null) {
            IconButton(onClick = onClick) {
                Icon(
                    imageVector = androidx.compose.material3.Icons.Default.KeyboardArrowRight,
                    contentDescription = null
                )
            }
        }
    }
}
