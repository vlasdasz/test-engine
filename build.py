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

if len(sys.argv) > 1:
    if sys.argv[1] == "ios":
        ios = True



def _get_home():
    if "HOME" in os.environ:
        return os.environ["HOME"]
    return os.path.expanduser("~")
    

home = _get_home()

tools_path = home + "/.rdeps/tools/"

def copy(src, dst):
    print("Copying:\n" + src + " to:\n" + dst)
    if os.path.isfile(src):
        shutil.copyfile(src, dst)
    else:
        shutil.copytree(src, dst)


def run(string):
    print(string)
    if os.system(string):
        print("Shell script has failed")
        exit()


def clone(rep, destination = ""):
    if not os.path.exists(destination):
        run("git clone --recursive https://github.com/vladasz/" + rep + " " + destination)


clone("tools", tools_path)


def link_tools():
    try:
        os.symlink(tools_path, "./tools")
    except FileExistsError:
        print("exists")

def linux_setup():
    print("Lin setup")
    run("sudo apt install mesa-common-dev libgl1-mesa-dev libglu1-mesa-dev xorg-dev")
    link_tools()


def windows_setup():
    print("Win setup")
    link_tools()


def mac_setup():
    print("Mac setup")
    link_tools()

if is_windows:
    windows_setup()
elif is_mac:
    mac_setup()
elif is_linux:
    linux_setup()
else:
    print("Unknown os")


if ios:
    run("rustup target add aarch64-apple-ios x86_64-apple-ios")
    run("cargo install cargo-lipo")
    run("cargo lipo")
    os.chdir("mobile/iOS")
    run("xcodebuild -showsdks")
    run("xcodebuild -sdk iphonesimulator -scheme TestEngine build")
else:
    run("cargo build --verbose")
