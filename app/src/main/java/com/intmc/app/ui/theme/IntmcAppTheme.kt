package com.intmc.app.ui.theme

import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import com.intmc.app.di.AppModuleLocal

@Composable
fun IntmcAppTheme(
    content: @Composable () -> Unit
) {
    val appModule = AppModuleLocal.current
    val context = appModule.context

    // Use dark theme by default for better OLED battery life on low-end devices
    val darkTheme = true

    Surface(
        modifier = Modifier.fillMaxSize(),
        color = MaterialTheme.colorScheme.background
    ) {
        IntmcTheme(darkTheme = darkTheme) {
            content()
        }
    }
}
