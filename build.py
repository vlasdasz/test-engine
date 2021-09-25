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
cleanup = False
android = False

if len(sys.argv) > 1:
    if sys.argv[1] == "ios":
        ios = True
    if sys.argv[1] == "android":
        android = True
    if sys.argv[1] == "cleanup":
        cleanup = True


def _get_home():
    if "HOME" in os.environ:
        return os.environ["HOME"]
    return os.path.expanduser("~")
    

home = _get_home()
deps_path = home + "/.rdeps/"

tools_path = deps_path + "tools/"
gles_path = deps_path + "gles31-sys/"
#soil_path = deps_path + "soil2/"
this_path = os.path.dirname(os.path.abspath(__file__))

def rm(path):
    print("Removing: " + path)
    if os.path.exists(path):
        if os.path.isfile(path):
            os.remove(path)
        else:
            shutil.rmtree(path)


if cleanup:
    rm(tools_path)
    rm(gles_path)


def run(string):
    print(string)
    if os.system(string):
        raise Exception("Shell script has failed")


def clone(rep, destination = ""):
    if not os.path.exists(destination):
        run("git clone --recursive https://github.com/vladasz/" + rep + " " + destination)


def setup_android():
    if os.path.isdir("NDK"):
        return
    run("mkdir NDK")
    run("rustup target add aarch64-linux-android armv7-linux-androideabi")
    ndk_home = "${ANDROID_HOME}/ndk/22.1.7171670"
    run(ndk_home + "/build/tools/make_standalone_toolchain.py --api 21 --arch arm64 --install-dir NDK/arm64")
    run(ndk_home + "/build/tools/make_standalone_toolchain.py --api 19 --arch arm --install-dir NDK/arm")


if android:
    setup_android()


#clone("soil2", soil_path)
clone("tools", tools_path)
clone("gles31-sys", gles_path)


def link_deps():
    try:
        print("Symlimk: " + deps_path + " to: " + this_path + "/.rdeps")
        os.symlink(deps_path, this_path + "/.rdeps")
    except FileExistsError:
        print("exists")

print("Arch:")
print(platform.uname())



def linux_setup():
    print("Lin setup")
    run("sudo apt update")
    run("sudo apt install mesa-common-dev libgl1-mesa-dev libglu1-mesa-dev xorg-dev")
    link_deps()


def windows_setup():
    print("Win setup")
    link_deps()


def mac_setup():
    print("Mac setup")
    link_deps()

if is_windows:
    windows_setup()
elif is_mac:
    mac_setup()
elif is_linux:
    linux_setup()
else:
    print("Unknown os")


if ios:
    os.environ["CARGO_CFG_TARGET_OS"] = "ios"
    run("rustup target add aarch64-apple-ios x86_64-apple-ios")
    run("cargo install cargo-lipo")
    run("cargo lipo --release")
    os.chdir("mobile/iOS")
    run("xcodebuild -showsdks")
    run("xcodebuild -sdk iphonesimulator -scheme TestEngine build")
elif android:
    os.environ["CARGO_CFG_TARGET_OS"] = "android"
    run("cargo build --target aarch64-linux-android --release --lib")
    run("cargo build --target armv7-linux-androideabi --release --lib")
else:
    os.environ["CARGO_CFG_TARGET_OS"] = "desktop"
    run("cargo build")
