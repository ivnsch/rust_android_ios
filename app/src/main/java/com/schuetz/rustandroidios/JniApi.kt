package com.schuetz.rustandroidios

class JniApi {
    init {
        try {
            System.loadLibrary("core")
        } catch (e: UnsatisfiedLinkError) {
            throw UnsatisfiedLinkError("Error linking Rust library. Check that the .so file " +
                "for the current architecture is in the libs directory. Error: $e")
        }
    }

    external fun initLogger()

    external fun add(value1: Int, value2: Int): Int
    external fun greet(who: String): String

    external fun passObject(dummy: Dummy)
    external fun returnObject(): Dummy

    external fun registerCallback(callback: Callback)
}

data class Dummy(
    val stringPar: String,
    val intPar: Int
)

interface Callback {
    fun call(string: String)
}
