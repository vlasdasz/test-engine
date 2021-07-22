cmake_minimum_required(VERSION 3.10.2)

project("test_engine")

set(PROJECT_NAME native-lib)


add_library(native-lib SHARED native-lib.cpp)


find_library(log-lib log)
target_link_libraries(native-lib ${log-lib})

find_library(android-lib android)
target_link_libraries(native-lib ${android-lib})

find_library(EGL GLESv2 GLESv3)
target_link_libraries(native-lib EGL GLESv2 GLESv3)

set(ENGINE_ROOT /Users/vladas/.rdeps/test_engine)


include_directories(${ENGINE_ROOT}/src)

link_directories(/Users/vladas/.rdeps/test_engine/target/armv7-linux-androideabi/release)

target_link_libraries(native-lib test_engine)



