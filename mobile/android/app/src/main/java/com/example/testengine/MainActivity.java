package com.example.testengine;

import androidx.appcompat.app.AppCompatActivity;

import android.content.res.AssetManager;
import android.os.Bundle;
import android.util.Log;
import android.view.MotionEvent;
import android.view.Window;
import android.view.WindowManager;

import java.io.IOException;
import java.io.InputStream;
import java.util.Arrays;

public class MainActivity extends AppCompatActivity {

    static {
        System.loadLibrary("test_game");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        AssetManager asset_manager = getResources().getAssets();

        setAssetManager(asset_manager);

        requestWindowFeature(Window.FEATURE_NO_TITLE);
        getWindow().setFlags(WindowManager.LayoutParams.FLAG_FULLSCREEN,
                WindowManager.LayoutParams.FLAG_FULLSCREEN);

        MyGLSurfaceView gLView = new MyGLSurfaceView(this);
        setContentView(gLView);
    }

    @Override
    public boolean onTouchEvent(MotionEvent event) {
        int actionIndex = event.getActionIndex();
        int actionId = event.getPointerId(actionIndex);
        int actionMasked = event.getActionMasked();
        switch (actionMasked) {
            case MotionEvent.ACTION_DOWN:
            case MotionEvent.ACTION_POINTER_DOWN:{
                onTouch(actionId, event.getX(actionIndex) / 2, event.getY(actionIndex) / 2, 0);
                return true;
            }
            case MotionEvent.ACTION_MOVE:{
                for(int i = 0; i < event.getPointerCount(); i++){
                    onTouch(event.getPointerId(i), event.getX(i) / 2, event.getY(i) / 2, 1);
                }
                return true;
            }
            case MotionEvent.ACTION_UP:
            case MotionEvent.ACTION_POINTER_UP: {
                onTouch(actionId, event.getX(actionIndex) / 2, event.getY(actionIndex) / 2, 2);
                return true;
            }
        }
        return super.onTouchEvent(event);
    }

    public native void setAssetManager(AssetManager assetManager);
    public native void onTouch(int id, float x, float y, int event);

}