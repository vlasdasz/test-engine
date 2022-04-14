package com.example.testengine;

import android.content.Context;
import android.opengl.GLSurfaceView;
import android.opengl.GLES30;
import android.util.DisplayMetrics;
import android.view.Display;
import android.view.WindowManager;

import javax.microedition.khronos.egl.EGLConfig;
import javax.microedition.khronos.opengles.GL10;

public class MyGLRenderer implements GLSurfaceView.Renderer {

    public Context context;

    @Override
    public void onSurfaceCreated(GL10 gl, EGLConfig config) {
        getMonitor();
    }

    @Override
    public void onDrawFrame(GL10 unused) {
        update();
    }

    @Override
    public void onSurfaceChanged(GL10 unused, int width, int height) {
        setScreenSize(width, height);
    }

    void getMonitor() {
        DisplayMetrics metrics = context.getResources().getDisplayMetrics();
        Display display = ((WindowManager) context.getSystemService(Context.WINDOW_SERVICE)).getDefaultDisplay();

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
        float width = (float)(sizeX * 25.4);
        float height = (float)(sizeY * 25.4);
        float diagonal = (float)screenInches;

        setMonitor(ppi, scale, refresh_rate, resolutionX, resolutionY, width, height, diagonal);
    }

    public native void update();
    public native void setScreenSize(int width, int height);
    public native void setMonitor(int ppi,
                                  float scale,
                                  int refresh_rate,
                                  int resolutionX,
                                  int resolutionY,
                                  float width,
                                  float height,
                                  float diagonal);

}
