package com.schuetz.rustandroidios

import android.os.Bundle
import android.support.v7.app.AppCompatActivity
import com.schuetz.rustandroidios.R.layout.activity_main

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(activity_main)

        val jniApi = JniApi()

        val greetResult = jniApi.greet("MyName")
        println("JNI greetResult: $greetResult")

        val addResult = jniApi.add(1, 2)
        println("JNI addResult: $addResult")

        val passClassResult = jniApi.passObject(Dummy("foo", 1))
        println("JNI passClassResult: $passClassResult")

        val returnClassResult = jniApi.returnObject()
        println("JNI returnClassResult: $returnClassResult")

        val myCallback = object: Callback {
            override fun call(string: String) {
                println("JNI callback called: $string")
            }
        }

        jniApi.registerCallback(myCallback)
    }
}
