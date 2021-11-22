package com.example.testengine;

import android.opengl.GLSurfaceView;
import android.opengl.GLES30;

import javax.microedition.khronos.egl.EGLConfig;
import javax.microedition.khronos.opengles.GL10;

public class MyGLRenderer implements GLSurfaceView.Renderer {

    @Override
    public void onSurfaceCreated(GL10 gl, EGLConfig config) {
        GLES30.glEnable(0);
       // GLES30.glGenVertexArrays(1, 1);
        //setup();
    }

    @Override
    public void onDrawFrame(GL10 unused) {
        //update();
    }

    @Override
    public void onSurfaceChanged(GL10 unused, int width, int height) {
        //setScreenSize(width, height);
    }

//    public native void setup();
//    public native void update();
//    public native void setScreenSize(int width, int height);

}
