package dev.eatsteak.rusaint.core

import android.content.Context
import androidx.startup.Initializer

class RusaintInitializer : Initializer<Unit> {
    override fun create(context: Context) {
        RusaintAndroid.initialize(context.applicationContext)
    }

    override fun dependencies(): List<Class<out Initializer<*>>> = emptyList()
}
