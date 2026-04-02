package dev.eatsteak.rusaint.core

import android.content.Context
import java.util.concurrent.atomic.AtomicBoolean

object RusaintAndroid {
    private val verifierInitialized = AtomicBoolean(false)

    init {
        System.loadLibrary("rusaint_ffi")
    }

    @JvmStatic
    private external fun nativeInitPlatformVerifier(context: Context): Boolean

    @JvmStatic
    fun initialize(context: Context) {
        if (verifierInitialized.get()) {
            return
        }

        synchronized(this) {
            if (verifierInitialized.get()) {
                return
            }

            check(nativeInitPlatformVerifier(context.applicationContext)) {
                "Failed to initialize rustls platform verifier"
            }
            verifierInitialized.set(true)
        }
    }
}
