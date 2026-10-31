package com.intmc.app.ui.theme

import androidx.compose.material3.ColorScheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme

private val DarkColorScheme = darkColorScheme(
    background = android.graphics.Color.BLACK,
    surface = android.graphics.Color.parseColor("#121212"),
    surfaceVariant = android.graphics.Color.parseColor("#1E1E1E"),
    primary = android.graphics.Color.parseColor("#BB86FC"),
    primaryVariant = android.graphics.Color.parseColor("#3700B3"),
    secondary = android.graphics.Color.parseColor("#03DAC6"),
    secondaryVariant = android.graphics.Color.parseColor("#03DAC6"),
    error = android.graphics.Color.parseColor("#CF6679"),
    onPrimary = android.graphics.Color.BLACK,
    onSecondary = android.graphics.Color.BLACK,
    onBackground = android.graphics.Color.WHITE,
    onSurface = android.graphics.Color.WHITE,
    onSurfaceVariant = android.graphics.Color.parseColor("#C0C0C0")
)

private val LightColorScheme = lightColorScheme(
    background = android.graphics.Color.WHITE,
    surface = android.graphics.Color.parseColor("#F5F5F5"),
    surfaceVariant = android.graphics.Color.parseColor("#E0E0E0"),
    primary = android.graphics.Color.parseColor("#6200EE"),
    secondary = android.graphics.Color.parseColor("#03DAC6"),
    error = android.graphics.Color.parseColor("#B00020"),
    onPrimary = android.graphics.Color.WHITE,
    onSecondary = android.graphics.Color.BLACK,
    onBackground = android.graphics.Color.BLACK,
    onSurface = android.graphics.Color.BLACK,
    onSurfaceVariant = android.graphics.Color.parseColor("#4A4A4A")
)

val darkColorScheme: ColorScheme = DarkColorScheme
val lightColorScheme: ColorScheme = LightColorScheme
