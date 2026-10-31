package com.intmc.app.di

import android.app.Application
import android.content.Context
import androidx.compose.runtime.CompositionLocal
import androidx.compose.runtime.ProvidableCompositionLocal
import com.intmc.app.data.repository.ServerRepository
import com.intmc.app.domain.backend.BackendRegistry
import com.intmc.app.domain.usecase.TunnelUseCase

class AppModule(private val application: Application) {

    val navController = androidx.navigation.NavController(application)

    fun provideAppModuleLocal(): ProvidableCompositionLocal<AppModule> {
        return AppModuleLocal
    }

    fun getRepository(context: Context): ServerRepository {
        return ServerRepository.getInstance(context)
    }

    fun getBackendRegistry(): BackendRegistry {
        return BackendRegistry
    }

    fun getTunnelUseCase(context: Context): TunnelUseCase {
        return TunnelUseCase(context)
    }

    val context: Context
        get() = application
}

object AppModuleLocal : ProvidableCompositionLocal<AppModule> {
    override val defaultValue: AppModule
        get() = throw IllegalStateException("AppModule not provided")
}
