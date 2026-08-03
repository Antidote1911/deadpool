cmake_minimum_required(VERSION 3.20)
project(deadpool-qt LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

find_package(Qt6 REQUIRED COMPONENTS Widgets)

# ── Build the Rust FFI static library via Cargo ───────────────────────────────

set(CARGO_TARGET_DIR "${CMAKE_BINARY_DIR}/cargo-target")

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_PROFILE "debug")
    set(CARGO_PROFILE_FLAG "")
else()
    set(CARGO_PROFILE "release")
    set(CARGO_PROFILE_FLAG "--release")
endif()

set(DEADPOOL_FFI_LIB "${CARGO_TARGET_DIR}/${CARGO_PROFILE}/libdeadpool_ffi.a")

add_custom_command(
    OUTPUT  "${DEADPOOL_FFI_LIB}"
    COMMAND cargo build ${CARGO_PROFILE_FLAG} -p deadpool-ffi
                        --target-dir "${CARGO_TARGET_DIR}"
    WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}/.."
    COMMENT "Building deadpool-ffi (Rust staticlib)"
    VERBATIM
)
add_custom_target(deadpool_ffi DEPENDS "${DEADPOOL_FFI_LIB}")

# ── Qt executable ─────────────────────────────────────────────────────────────

qt_add_executable(deadpool-qt
    src/main.cpp
    src/MainWindow.cpp
    src/MainWindow.ui
    resources.qrc
)

add_dependencies(deadpool-qt deadpool_ffi)

target_include_directories(deadpool-qt PRIVATE
    "${CMAKE_SOURCE_DIR}/../deadpool-ffi/include"
)

target_link_libraries(deadpool-qt PRIVATE
    Qt6::Widgets
    "${DEADPOOL_FFI_LIB}"
    pthread
    dl
)

# Warnings
target_compile_options(deadpool-qt PRIVATE
    -Wall -Wextra -Wpedantic
)
