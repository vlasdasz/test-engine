#!/usr/bin/env python3

import os
import sys
import glob
import shutil
import platform
import subprocess
import urllib.request

is_windows = platform.system() == "Windows"
is_mac     = platform.system() == "Darwin"
is_linux   = platform.system() == "Linux"

unix = is_mac or is_linux

ios = False
android = False


def get_uname():
    if unix:
        return str(subprocess.check_output(['uname', '-a']).lower())
    else:
        return ""


def run(string):
    print(string)
    if os.system(string):
        raise Exception("Shell script has failed")


uname = get_uname()

is_fedora = "fedora" in uname
is_arch = "arch" in uname

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


def setup_android():

    print("Add rust targets")
    run("rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android")

    platform = "linux" if is_linux else "darwin"
    arch_platform = platform + "-x86_64"
    bin = this_path + "/ndk/bin"
    version = "r22b"
    api_level = "21"

    toolchains = "/ndk/android-ndk-" + version + "/toolchains/"

    os.environ["NDK_INCLUDE_DIR"] = this_path + toolchains + "llvm/prebuilt/" + arch_platform + "/sysroot/usr/include"
    os.environ["PATH"] += ":" + bin

    if os.path.isdir("ndk"):
        print("NDK directory already exists")
        return

    run("mkdir ndk")

    print("Downloading NDK")

    urllib.request.urlretrieve("https://dl.google.com/android/repository/android-ndk-" + version + "-" + arch_platform + ".zip", "ndk/ndk.zip")
    shutil.unpack_archive("ndk/ndk.zip", "ndk")

    print("Symlink NDK bin")

    os.symlink(this_path + toolchains + "llvm/prebuilt/" + arch_platform + "/bin", bin)

    print("Symlink clang")
    shutil.copyfile(bin + "/aarch64-linux-android" + api_level + "-clang",
                    bin + "/aarch64-linux-android-clang")
    shutil.copyfile(bin + "/aarch64-linux-android" + api_level + "-clang++",
                    bin + "/aarch64-linux-android-clang++")
    shutil.copyfile(bin + "/llvm-ar",
                    bin + "/aarch64-linux-android-ar")

    shutil.copyfile(bin + "/armv7a-linux-androideabi" + api_level + "-clang",
                    bin + "/arm-linux-androideabi-clang")
    shutil.copyfile(bin + "/armv7a-linux-androideabi" + api_level + "-clang++",
                    bin + "/arm-linux-androideabi-clang++")

    for file in glob.glob(bin + "/*"):
        run("sudo chmod +x " + file)


def build_android():

    run("cargo build --target aarch64-linux-android --release --lib")
    # run("cargo build --target armv7-linux-androideabi --release --lib")
    # run("cargo build --target i686-linux-android --release --lib")

    run("mkdir -p mobile/android/app/src/main/jniLibs/")
    run("mkdir -p mobile/android/app/src/main/jniLibs/arm64-v8a")
    run("mkdir -p mobile/android/app/src/main/jniLibs/armeabi-v7a")
    run("mkdir -p mobile/android/app/src/main/jniLibs/x86")

    try:
        os.symlink(this_path + "/target/aarch64-linux-android/release/libtest_game.so",
                   "mobile/android/app/src/main/jniLibs/arm64-v8a/libtest_game.so")

        # os.symlink(this_path + "/target/armv7-linux-androideabi/release/libtest_game.so", "mobile/android/app/src/main/jniLibs/armeabi-v7a/libtest_game.so")
        # os.symlink(this_path + "/target/i686-linux-android/release/libtest_game.so", "mobile/android/app/src/main/jniLibs/x86/libtest_game.so")
    except FileExistsError:
        print("exists")


def build_ios():
    run("rustup target add aarch64-apple-ios x86_64-apple-ios")
    run("cargo install cargo-lipo")
    run("cargo lipo --release")
    os.chdir("mobile/iOS")
    run("xcodebuild -showsdks")
    run("xcodebuild -sdk iphonesimulator -scheme TestEngine build")


print("Arch:")
print(platform.uname())


if is_linux and desktop:
    print("Lin setup")

    run("curl https://sh.rustup.rs -sSf | sh -s -- -y")

    if is_fedora:
        run("sudo dnf update")
        run("sudo dnf install libXcursor-devel libXi-devel libXinerama-devel libXrandr-devel alsa-lib-devel-1.2.6.1-3.fc34.aarch64")
    elif is_arch:
        print("Arch")
    else:
        run("sudo apt update")
        run("sudo apt -y install cmake mesa-common-dev libgl1-mesa-dev libglu1-mesa-dev xorg-dev libasound2-dev")

if ios:
    build_ios()
elif android:
    setup_android()
    build_android()
else:
    run("cargo build")
