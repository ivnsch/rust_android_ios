package com.schuetz.rustandroidios

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.Assert.assertEquals
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class ExampleInstrumentedTest {

    @Test
    fun greet() {
        val res = JniApi().greet("Ivan")
        assertEquals("Hello \uD83D\uDC4B Ivan!", res)
    }

    @Test
    fun add() {
        val res = JniApi().add(1, 2)
        assertEquals(3, res)
    }

    @Test
    fun passClass() {
        JniApi().passObject(Dummy("sfds", 2))
        // There's no result. Only testing that it doesn't crash.
    }

    @Test
    fun returnClass() {
        val res = JniApi().returnObject()
        assertEquals(Dummy("my string parameter", 123), res)
    }

    @Test
    fun registersCallback() {
        JniApi().registerCallback(object : Callback {
            override fun call(string: String) {
                // Testing callbacks left as an exerice. This requires new dependencies and
                // it's not relatd with Rust.
                println("Callback called with: $string")
            }
        })
    }
}
