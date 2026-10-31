buildscript {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
    dependencies {
        classpath("com.android.tools.build:gradle:8.4.0")
        classpath("org.jetbrains.kotlin:kotlin-gradle-plugin:1.9.22")
        classpath("androidx.navigation:navigation-safe-args-gradle-plugin:2.7.7")
    }
}

import org.gradle.api.tasks.Delete

plugins {
    id("com.android.application") version "8.4.0" apply false
    id("org.jetbrains.kotlin.android") version "1.9.22" apply false
}

allprojects {
    configurations.matching { it.name == "releaseRuntimeProps" || it.name == "debugRuntimeProps" }.all {
        resolutionStrategy.dependencySubstitution {
            // substitute(module("androidx.activity:activity")).`with`(module("androidx.activity:activity:1.8.2"))
        }
    }
}

tasks.register("clean", Delete::class) {
    delete(rootProject.buildDir)
}
