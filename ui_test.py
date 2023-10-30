#!/usr/bin/env python3

import os
import time
import platform

is_windows = platform.system() == "Windows"
is_mac = platform.system() == "Darwin"
is_linux = platform.system() == "Linux"

unix = is_mac or is_linux

ios = False
android = False

this_path = os.path.dirname(os.path.abspath(__file__))


def run(string):
    print(string)
    time.sleep(0.1)
    if os.system(string):
        raise Exception("Shell script has failed")


def get_files_from_directory(directory_path):
    result = []
    for file in os.listdir(directory_path):
        if os.path.isfile(os.path.join(directory_path, file)):
            result.append(file)
    return result


# Example usage:
directory_path = 'ui_test/src'
files = get_files_from_directory(directory_path)
files = [file.split('.')[0] for file in files]


for file in files:
    if file == "main":
        continue

    run("cargo run -p ui_test --bin " + file + "_test")
    run("cargo run -p ui_test --bin " + file + "_test --release")
