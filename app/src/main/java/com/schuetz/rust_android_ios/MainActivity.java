package com.schuetz.rust_android_ios;

import android.os.Bundle;
import android.support.v7.app.AppCompatActivity;
import android.support.v7.widget.Toolbar;
import android.util.Log;
import android.widget.TextView;

public class MainActivity extends AppCompatActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        Toolbar toolbar = findViewById(R.id.toolbar);
        setSupportActionBar(toolbar);

        try {
            System.loadLibrary("mobcore");
        } catch (UnsatisfiedLinkError e) {
            Log.e("MainActivity", "Load library error: " + e);
            return;
        }

        final MyRustStruct myRustStruct = new MyRustStruct();

        final TextView greetingTextView = findViewById(R.id.greetingLabel);
        greetingTextView.setText(myRustStruct.greet("Ivan"));

        final TextView numberTextView = findViewById(R.id.numberLabel);
        final int res = myRustStruct.add(2);
        numberTextView.setText(getString(R.string.rust_add_text, res));

        final TextView callbackTextView = findViewById(R.id.callbackLabel);
        myRustStruct.function_with_callback(new Callback() {
            @Override
            public void call(final int a_number, final boolean a_boolean) {
            callbackTextView.setText("Got callback result: a_number: " + a_number + ", a_boolean: " + a_boolean);
            }
        });

        final TextView eventsTextView = findViewById(R.id.eventsLabel);
        myRustStruct.observe(new Callback() {
            @Override
            public void call(final int a_number, final boolean a_boolean) {
                eventsTextView.post(new Runnable() {
                    @Override
                    public void run() {
                        eventsTextView.setText("Received event: a_number: " + a_number + ", a_boolean: " + a_boolean);
                    }
                });
            }
        });
        myRustStruct.send_to_observers(1);
        eventsTextView.postDelayed(new Runnable() {
            @Override
            public void run() {
                myRustStruct.send_to_observers(2);
            }
        }, 2000);
    }
}
