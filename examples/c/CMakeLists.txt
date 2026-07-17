# Build and run the C and C++ examples against the wickra-timemachine C ABI.
#
#   cargo build --release -p wickra-timemachine-c
#   cmake -S examples/c -B examples/c/build
#   cmake --build examples/c/build --config Release
#   ctest --test-dir examples/c/build -C Release --output-on-failure
#
# WICKRA_TIMEMACHINE_LIB_DIR defaults to the workspace target/release directory and can
# be overridden for out-of-tree builds.

cmake_minimum_required(VERSION 3.15)
project(wickra_timemachine_examples C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(WICKRA_TIMEMACHINE_LIB_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../target/release"
    CACHE PATH "Directory containing the built wickra-timemachine C ABI library")

set(WICKRA_TIMEMACHINE_INCLUDE "${CMAKE_CURRENT_SOURCE_DIR}/../../bindings/c/include")

find_library(WICKRA_TIMEMACHINE_LIB
    NAMES wickra_timemachine libwickra_timemachine
    HINTS "${WICKRA_TIMEMACHINE_LIB_DIR}"
    REQUIRED)

add_executable(seek seek.c)
target_include_directories(seek PRIVATE "${WICKRA_TIMEMACHINE_INCLUDE}")
target_link_libraries(seek PRIVATE "${WICKRA_TIMEMACHINE_LIB}")

add_executable(seekpp seek.cpp)
target_include_directories(seekpp PRIVATE "${WICKRA_TIMEMACHINE_INCLUDE}")
target_link_libraries(seekpp PRIVATE "${WICKRA_TIMEMACHINE_LIB}")

# Both examples build the demo sine universe with sin() from <math.h>. On Linux
# libm is a separate object that ld does not pull in implicitly, so link it
# explicitly. macOS folds libm into libSystem and Windows into the CRT, where
# the linker resolves it without a flag — but linking m there is a harmless
# no-op, so guard only against Windows.
if(NOT WIN32)
    target_link_libraries(seek PRIVATE m)
    target_link_libraries(seekpp PRIVATE m)
endif()

# On Windows the runtime DLL must sit next to the executable (there is no rpath);
# without it the loader pops a modal "DLL not found" dialog that hangs ctest. Copy
# it next to each example after build. (Unix resolves the .so/.dylib via the
# linker's embedded path, so this is Windows-only.)
if(WIN32)
    foreach(example seek seekpp)
        add_custom_command(TARGET ${example} POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
                "${WICKRA_TIMEMACHINE_LIB_DIR}/wickra_timemachine.dll"
                "$<TARGET_FILE_DIR:${example}>"
            VERBATIM)
    endforeach()
endif()

enable_testing()
add_test(NAME seek COMMAND seek)
add_test(NAME seekpp COMMAND seekpp)
# A missing runtime dependency must fail fast, never block the runner for hours.
set_tests_properties(seek seekpp PROPERTIES TIMEOUT 60)
