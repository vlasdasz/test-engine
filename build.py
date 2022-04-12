#!/usr/bin/env python3

import os
import sys
import glob
import shutil
import platform
import urllib.request

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

mobile = ios or android
desktop = not mobile


def get_home():
    if "HOME" in os.environ:
        return os.environ["HOME"]
    return os.path.expanduser("~")
    

home = get_home()

this_path = os.path.dirname(os.path.abspath(__file__))


def run(string):
    print(string)
    if os.system(string):
        raise Exception("Shell script has failed")


def setup_android():

    if os.path.isdir("ndk"):
        print("NDK directory already exists")
        return

    bin = this_path + "/ndk/bin"

    run("mkdir ndk")

    print("Downloading NDK")
    if is_linux:
        urllib.request.urlretrieve("https://dl.google.com/android/repository/android-ndk-r22b-linux-x86_64.zip", "ndk/ndk.zip")
        shutil.unpack_archive("ndk/ndk.zip", "ndk")
    elif is_mac:
        urllib.request.urlretrieve("https://dl.google.com/android/repository/android-ndk-r22b-darwin-x86_64.zip", "ndk/ndk.zip")
        shutil.unpack_archive("ndk/ndk.zip", "ndk")

    toolchains = "/ndk/android-ndk-r22b/toolchains/"

    print("Symlink NDK bin")
    if is_linux:
        os.symlink(this_path + toolchains + "llvm/prebuilt/linux-x86_64/bin", bin)
        os.environ["NDK_INCLUDE_DIR"] = this_path + toolchains + "llvm/prebuilt/linux-x86_64/sysroot/usr/include"
    elif is_mac:
        os.symlink(this_path + toolchains + "llvm/prebuilt/darwin-x86_64/bin", bin)
        os.environ["NDK_INCLUDE_DIR"] = this_path + toolchains + "llvm/prebuilt/darwin-x86_64/sysroot/usr/include"

    print("Symlink clang")
    shutil.copyfile(bin + "/aarch64-linux-android21-clang", 
                    bin + "/aarch64-linux-android-clang")
    shutil.copyfile(bin + "/aarch64-linux-android21-clang++", 
                    bin + "/aarch64-linux-android-clang++")

    shutil.copyfile(bin + "/armv7a-linux-androideabi21-clang", 
                    bin + "/arm-linux-androideabi-clang")
    shutil.copyfile(bin + "/armv7a-linux-androideabi21-clang++", 
                    bin + "/arm-linux-androideabi-clang++")

    for file in glob.glob(bin + "/*"):
        run("sudo chmod +x " + file)

    os.environ["PATH"] += ":" + bin
 
    print("Add rust targets")
    run("rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android")


def build_android():

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


def build_ios():
    run("rustup target add aarch64-apple-ios x86_64-apple-ios ")
    run("cargo install cargo-lipo")
    run("cargo lipo --release")
    os.chdir("mobile/iOS")
    run("xcodebuild -showsdks")
    run("xcodebuild -sdk iphonesimulator -scheme TestEngine build")


print("Arch:")
print(platform.uname())

if is_linux and desktop:
    print("Lin setup")
    run("sudo apt update")
    run("sudo apt -y install cmake mesa-common-dev libgl1-mesa-dev libglu1-mesa-dev xorg-dev libasound2-dev")
                                                  
if ios:
    build_ios()
elif android:
    setup_android()
    build_android()
else:
    run("cargo build")
