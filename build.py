#!/usr/bin/env python3

import os
import sys
import shutil
import platform

is_windows = platform.system() == "Windows"
is_mac     = platform.system() == "Darwin"
is_linux   = platform.system() == "Linux"

unix = is_mac or is_linux

ios = False
android = False

if len(sys.argv) > 1:
    if sys.argv[1] == "ios":
        ios = True
    if sys.argv[1] == "android":
        android = True


def _get_home():
    if "HOME" in os.environ:
        return os.environ["HOME"]
    return os.path.expanduser("~")
    

home = _get_home()

this_path = os.path.dirname(os.path.abspath(__file__))


def run(string):
    print(string)
    if os.system(string):
        raise Exception("Shell script has failed")


def ndk_home():
    if "NDK_HOME" in os.environ:
        return os.environ["NDK_HOME"]
    if "ANDROID_HOME" in os.environ:
        return os.environ["ANDROID_HOME"] + "/ndk/22.1.7171670"
    return "/Users/vladas/Library/Android/sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin"
    raise Exception("No NDK path env variables")


def setup_android():
    run("rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android")
    print("skigal")
    print(this_path)
    print(ndk_home())
    try:
        os.symlink(ndk_home(), this_path + "/NDK")
    except FileExistsError:
        print("NDK symlink exists")

if android:
    setup_android()


print("Arch:")
print(platform.uname())


if is_linux:
    print("Lin setup")
    run("sudo apt update")
    run("sudo apt -y install cmake mesa-common-dev libgl1-mesa-dev libglu1-mesa-dev xorg-dev")


if ios:
    os.environ["CARGO_CFG_TARGET_OS"] = "ios"
    run("rustup target add aarch64-apple-ios x86_64-apple-ios ")
    run("cargo install cargo-lipo")
    run("cargo lipo --release")
    os.chdir("mobile/iOS")
    run("xcodebuild -showsdks")
    run("xcodebuild -sdk iphonesimulator -scheme TestEngine build")
elif android:
    os.environ["CARGO_CFG_TARGET_OS"] = "android"
    run("cargo build --target aarch64-linux-android --release --lib")
    run("cargo build --target armv7-linux-androideabi --release --lib")
    run("cargo build --target i686-linux-android --release --lib")

    run("mkdir -p mobile/android/app/src/main/jniLibs/")
    run("mkdir -p mobile/android/app/src/main/jniLibs/arm64-v8a")
    run("mkdir -p mobile/android/app/src/main/jniLibs/armeabi-v7a")
    run("mkdir -p mobile/android/app/src/main/jniLibs/x86")

    try:
        os.symlink(this_path + "/target/aarch64-linux-android/release/libtest_game.so", "mobile/android/app/src/main/jniLibs/arm64-v8a/libtest_game.so")
        os.symlink(this_path + "/target/armv7-linux-androideabi/release/libtest_game.so", "mobile/android/app/src/main/jniLibs/armeabi-v7a/libtest_game.so")
        os.symlink(this_path + "/target/i686-linux-android/release/libtest_game.so", "mobile/android/app/src/main/jniLibs/x86/libtest_game.so")
    except FileExistsError:
        print("exists")
else:
    run("cargo build")
