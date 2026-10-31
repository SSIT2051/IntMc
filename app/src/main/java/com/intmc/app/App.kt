package com.intmc.app

import android.app.Application
import com.intmc.app.data.local.AppDatabase
import com.intmc.app.data.repository.ServerRepository
import com.intmc.app.di.AppModule
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch

class App : Application() {

    lateinit var appModule: AppModule

    override fun onCreate() {
        super.onCreate()
        appModule = AppModule(this)
    }
}
