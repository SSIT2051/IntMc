package com.intmc.app

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.ui.Modifier
import com.intmc.app.di.AppModule
import com.intmc.app.ui.theme.IntmcTheme
import com.intmc.app.ui.screens.MainScreen

class MainActivity : ComponentActivity() {

    private lateinit var appModule: AppModule

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        appModule = (application as App).appModule

        setContent {
            CompositionLocalProvider(
                appModule.provideAppModuleLocal()
            ) {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    MainScreen(navController = appModule.navController)
                }
            }
        }
    }
}
