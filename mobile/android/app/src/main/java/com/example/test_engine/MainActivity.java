package com.example.test_engine;

import androidx.appcompat.app.AppCompatActivity;

import android.app.Activity;
import android.os.Bundle;
import android.util.Log;
import android.view.SurfaceHolder;
import android.view.SurfaceView;
import android.view.Window;
import android.view.WindowManager;
import android.widget.TextView;
import android.view.MotionEvent;

import android.util.Log;


import android.content.res.AssetManager;

public class MainActivity extends Activity {

    private MyGLSurfaceView gLView;

    static { System.loadLibrary("native-lib"); }

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
        setAssetManager(asset_manager);
    }

    @Override
    public boolean onTouchEvent(MotionEvent event) {
        int actionIndex = event.getActionIndex();
        int actionId = event.getPointerId(actionIndex);
        int actionMasked = event.getActionMasked();
        switch (actionMasked) {
            case MotionEvent.ACTION_DOWN:
            case MotionEvent.ACTION_POINTER_DOWN:{
                touchBegan(event.getX(actionIndex), event.getY(actionIndex), actionId);
                return true;
            }
            case MotionEvent.ACTION_MOVE:{
                for(int i = 0; i < event.getPointerCount(); i++){
                    touchMoved(event.getX(i), event.getY(i), event.getPointerId(i));
                }
                return true;
            }
            case MotionEvent.ACTION_UP:
            case MotionEvent.ACTION_POINTER_UP: {
                touchEnded(event.getX(actionIndex), event.getY(actionIndex), actionId);
                return true;
            }
        }
        return super.onTouchEvent(event);
    }


    public native void setAssetManager(AssetManager assetManager);
    public native void touchBegan(float x, float y, int id);
    public native void touchMoved(float x, float y, int id);
    public native void touchEnded(float x, float y, int id);


}
