package com.example.test_engine;

import javax.microedition.khronos.egl.EGLConfig;
import javax.microedition.khronos.opengles.GL10;

import android.opengl.GLSurfaceView;
import android.util.Log;

public class MyGLRenderer implements GLSurfaceView.Renderer {

    public void onSurfaceCreated(GL10 unused, EGLConfig config) {
        setup();
    }

    public void onDrawFrame(GL10 unused) {
        update();
    }

    public void onSurfaceChanged(GL10 unused, int width, int height) {
        setScreenSize(width, height);
    }

    public native void setup();
    public native void update();
    public native void setScreenSize(int width, int height);

}