package com.example.testengine;

import android.content.Context;
import android.opengl.GLSurfaceView;

import com.example.testengine.MyGLRenderer;

public class MyGLSurfaceView extends GLSurfaceView {

    private final MyGLRenderer renderer;

    public MyGLSurfaceView(Context context){
        super(context);
        setEGLContextClientVersion(3);
        renderer = new MyGLRenderer();
        setRenderer(renderer);
    }
}