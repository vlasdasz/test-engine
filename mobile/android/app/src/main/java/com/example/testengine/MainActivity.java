package com.example.testengine;

import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.util.Log;

public class MainActivity extends AppCompatActivity {

    static {
        System.loadLibrary("test_game");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        RustGreetings g = new RustGreetings();
        String r = "figok2" + g.sayHello("world");

        Log.d("fergel", r);
    }
}