package com.example.testengine;

import androidx.appcompat.app.AppCompatActivity;

import android.content.res.AssetManager;
import android.os.Bundle;
import android.util.Log;
import android.view.Window;
import android.view.WindowManager;

public class MainActivity extends AppCompatActivity {

    static {
        System.loadLibrary("test_game");
    }

    private MyGLSurfaceView gLView;
    private AssetManager asset_manager = null;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        requestWindowFeature(Window.FEATURE_NO_TITLE);
        getWindow().setFlags(WindowManager.LayoutParams.FLAG_FULLSCREEN,
                WindowManager.LayoutParams.FLAG_FULLSCREEN);

        gLView = new MyGLSurfaceView(this);
        setContentView(gLView);

        asset_manager = getResources().getAssets();
        //setAssetManager(asset_manager);
    }

//    @Override
//    protected void onCreate(Bundle savedInstanceState) {
//        super.onCreate(savedInstanceState);
//        setContentView(R.layout.activity_main);
//
//        RustGreetings g = new RustGreetings();
//        String r = "figok2" + g.sayHello("world");
//
//        Log.d("fergel", r);
//    }
}