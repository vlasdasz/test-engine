package com.example.testengine;

import androidx.appcompat.app.AppCompatActivity;

import android.content.Context;
import android.content.res.AssetManager;
import android.os.Bundle;
import android.util.DisplayMetrics;
import android.util.Log;
import android.view.Display;
import android.view.MotionEvent;
import android.view.View;
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
    public void onWindowFocusChanged(boolean hasFocus) {
        super.onWindowFocusChanged(hasFocus);
        hideSystemUI();
    }

    private void hideSystemUI() {
        View decorView = getWindow().getDecorView();
        decorView.setSystemUiVisibility(
                View.SYSTEM_UI_FLAG_IMMERSIVE
                        | View.SYSTEM_UI_FLAG_LAYOUT_STABLE
                        | View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION
                        | View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN
                        | View.SYSTEM_UI_FLAG_HIDE_NAVIGATION
                        | View.SYSTEM_UI_FLAG_FULLSCREEN
        );
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        getMonitor();
        setAssetManager(getResources().getAssets());
        setContentView(new MyGLSurfaceView(this));
    }

    @Override
    public boolean onTouchEvent(MotionEvent event) {
        int actionIndex = event.getActionIndex();
        int actionId = event.getPointerId(actionIndex);
        int actionMasked = event.getActionMasked();
        switch (actionMasked) {
            case MotionEvent.ACTION_DOWN:
            case MotionEvent.ACTION_POINTER_DOWN:{
                onTouch(actionId, event.getX(actionIndex), event.getY(actionIndex), 0);
                return true;
            }
            case MotionEvent.ACTION_MOVE:{
                for(int i = 0; i < event.getPointerCount(); i++){
                    onTouch(event.getPointerId(i), event.getX(i), event.getY(i), 1);
                }
                return true;
            }
            case MotionEvent.ACTION_UP:
            case MotionEvent.ACTION_POINTER_UP: {
                onTouch(actionId, event.getX(actionIndex), event.getY(actionIndex), 2);
                return true;
            }
        }
        return super.onTouchEvent(event);
    }

    void getMonitor() {
        DisplayMetrics metrics = getResources().getDisplayMetrics();
        Display display = ((WindowManager) getSystemService(Context.WINDOW_SERVICE)).getDefaultDisplay();

        double sizeX = metrics.widthPixels / metrics.xdpi;
        double sizeY = metrics.heightPixels / metrics.ydpi;

        double x = Math.pow(sizeX, 2);
        double y = Math.pow(sizeY, 2);
        double screenInches = Math.sqrt(x + y);

        int ppi = metrics.densityDpi;
        float scale = (int)metrics.density;
        int refresh_rate = (int)display.getRefreshRate();
        int resolutionX = metrics.widthPixels;
        int resolutionY = metrics.heightPixels;
        float width = (float)sizeX;
        float height = (float)sizeY;
        float diagonal = (float)screenInches;

        setMonitor(ppi, scale, refresh_rate, resolutionX, resolutionY, width, height, diagonal);
    }

    public native void setAssetManager(AssetManager assetManager);
    public native void onTouch(int id, float x, float y, int event);
    public native void setMonitor(int ppi,
                                  float scale,
                                  int refresh_rate,
                                  int resolutionX,
                                  int resolutionY,
                                  float width,
                                  float height,
                                  float diagonal);

}