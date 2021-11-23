package com.example.testengine;

import androidx.appcompat.app.AppCompatActivity;

import android.content.res.AssetManager;
import android.os.Bundle;
import android.util.Log;
import android.view.Window;
import android.view.WindowManager;

import java.io.IOException;
import java.io.InputStream;
import java.util.Arrays;

public class MainActivity extends AppCompatActivity {

    static {
        System.loadLibrary("test_game");
    }

    private MyGLSurfaceView gLView;
    private AssetManager asset_manager = null;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        asset_manager = getResources().getAssets();


        try {

            Log.d("test_engine", asset_manager.toString());


            String[] fonts = asset_manager.list("Fonts");

            Log.d("test_engine", Arrays.toString(fonts));

            for(String name:fonts){
                Log.d("test_engine", name);
            }
            InputStream file = asset_manager.open("Shaders/ui/ui.vert");
            Log.d("test_engine", file.toString());
            Log.d("test_engine", "izi opened");
        }
        catch(IOException e) {
            Log.d("test_engine", "axenfurt");
            Log.d("test_engine", e.toString());
        }


        //Log.d("spes", asset_manager);

        setAssetManager(asset_manager);

        requestWindowFeature(Window.FEATURE_NO_TITLE);
        getWindow().setFlags(WindowManager.LayoutParams.FLAG_FULLSCREEN,
                WindowManager.LayoutParams.FLAG_FULLSCREEN);

        gLView = new MyGLSurfaceView(this);
        setContentView(gLView);

    }

    public native void setAssetManager(AssetManager assetManager);

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