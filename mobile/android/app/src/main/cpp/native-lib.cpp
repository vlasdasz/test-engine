
#include <string>
#include <thread>

#include <jni.h>
#include <android/asset_manager_jni.h>
#include "test_engine.h"


extern "C" JNIEXPORT void JNICALL
Java_com_example_test_1engine_MyGLRenderer_setup(JNIEnv* env, jobject) {

    static bool not_first_call = false;

    if (not_first_call) {
        return;
    }

    create_screen();

}

extern "C" JNIEXPORT void JNICALL
Java_com_example_test_1engine_MyGLRenderer_update(JNIEnv* env, jobject) {
//    update_screen();
}

extern "C" JNIEXPORT void JNICALL
Java_com_example_test_1engine_MyGLRenderer_setScreenSize(JNIEnv* env, jobject, jint width, jint height) {
    //set_screen_size(width, height);
}

extern "C" JNIEXPORT void JNICALL
Java_com_example_test_1engine_MainActivity_setAssetManager(JNIEnv* env, jobject, jobject asset_manager) {

//    AAssetManager* manager = nullptr;
//    std::string error;
//
//    try {
//        manager = AAssetManager_fromJava(env, asset_manager);
//    }
//    catch (...) {
//        error = what();
//        Fatal(error);
//    }
//
//    if (manager == nullptr) {
//        Fatal("Failed to get asset manager.");
//    }
//
//    AndroidSystem::set_asset_manager(manager);
}

extern "C" JNIEXPORT void JNICALL
Java_com_example_test_1engine_MainActivity_touchBegan(JNIEnv *env, jobject, jfloat x, jfloat y, jint id) {
//    cu::Dispatch::on_main([=]{
//        ui::Input::process_touch_event(new ui::Touch(id, {x, y}, ui::Touch::Began));
//    });
}

extern "C" JNIEXPORT void JNICALL
Java_com_example_test_1engine_MainActivity_touchMoved(JNIEnv *env, jobject, jfloat x, jfloat y, jint id) {
//    cu::Dispatch::on_main([=]{
//        ui::Input::process_touch_event(new ui::Touch(id, {x, y}, ui::Touch::Moved));
//    });
}

extern "C" JNIEXPORT void JNICALL
Java_com_example_test_1engine_MainActivity_touchEnded(JNIEnv *env, jobject, jfloat x, jfloat y, jint id) {
//    cu::Dispatch::on_main([=]{
//        ui::Input::process_touch_event(new ui::Touch(id, {x, y}, ui::Touch::Ended));
//    });
}