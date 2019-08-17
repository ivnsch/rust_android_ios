package com.schuetz.rust_android_ios

import android.os.Bundle
import android.support.v7.app.AppCompatActivity
import android.support.v7.widget.Toolbar
import android.util.Log
import android.widget.TextView

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        val toolbar = findViewById<Toolbar>(R.id.toolbar)
        setSupportActionBar(toolbar)

        try {
            System.loadLibrary("mobcore")
        } catch (e: UnsatisfiedLinkError) {
            Log.e("MainActivity", "Load library error: $e")
            return
        }

        val myRustStruct = MyRustStruct()

        val greetingTextView = findViewById<TextView>(R.id.greetingLabel)
        greetingTextView.text = myRustStruct.greet("Ivan")

        val numberTextView = findViewById<TextView>(R.id.numberLabel)
        val res = myRustStruct.add(2)
        numberTextView.text = getString(R.string.rust_add_text, res)

        val callbackTextView = findViewById<TextView>(R.id.callbackLabel)
        myRustStruct.function_with_callback { a_number, a_boolean ->
            callbackTextView.text = "Got callback result: a_number: $a_number, a_boolean: $a_boolean"
        }

        val eventsTextView = findViewById<TextView>(R.id.eventsLabel)
        myRustStruct.observe { a_number, a_boolean ->
            eventsTextView.post {
                eventsTextView.text = "Received event: a_number: $a_number, a_boolean: $a_boolean"
            }
        }
        myRustStruct.send_to_observers(1)
        eventsTextView.postDelayed({ myRustStruct.send_to_observers(2) }, 2000)
    }
}
