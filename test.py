#!/usr/bin/env python3

import os
import time
import platform


is_windows = platform.system() == "Windows"
is_mac     = platform.system() == "Darwin"
is_linux   = platform.system() == "Linux"

unix = is_mac or is_linux

ios = False
android = False

this_path = os.path.dirname(os.path.abspath(__file__))

def run(string):
    print(string)
    time.sleep(0.1)
    if os.system(string):
        raise Exception("Shell script has failed")


run("cargo test --all")
run("cargo test -p valid")

os.chdir("deps/ui/ui_views/")
run("cargo test --all")
os.chdir(this_path)

os.chdir("deps/text/")
run("cargo test --all")
os.chdir(this_path)
