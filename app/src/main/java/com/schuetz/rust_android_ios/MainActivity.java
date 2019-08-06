package com.schuetz.rust_android_ios;

import android.os.Bundle;
import android.support.v7.app.AppCompatActivity;
import android.support.v7.widget.Toolbar;
import android.widget.TextView;

public class MainActivity extends AppCompatActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        Toolbar toolbar = findViewById(R.id.toolbar);
        setSupportActionBar(toolbar);

        final TextView greetingTextView = findViewById(R.id.greetingLabel);
        greetingTextView.setText(Session.greet("Ivan"));

        final TextView numberTextView = findViewById(R.id.numberLabel);
        final int res = MyApplication.get().getSession().add_and1(2);
        numberTextView.setText(getString(R.string.rust_add1_text, res));
    }
}
