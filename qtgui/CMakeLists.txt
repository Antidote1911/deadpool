cmake_minimum_required(VERSION 3.19)

project(qtpassgen LANGUAGES CXX)

set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
qt_standard_project_setup()

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 6.5 REQUIRED COMPONENTS Widgets)

if (WIN32)
  if (CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
    # using Clang
  elseif (CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
    # using GCC
    add_compile_options(-Wa,-mbig-obj)
  elseif (CMAKE_CXX_COMPILER_ID STREQUAL "Intel")
    # using Intel C++
  elseif (CMAKE_CXX_COMPILER_ID STREQUAL "MSVC")
    # using Visual Studio C++
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /Zc:__cplusplus")
    add_compile_options(/bigobj)
  endif()
endif (WIN32)

set(PROJECT_SOURCES
        main.cpp
        mainwindow.cpp
        mainwindow.h
        mainwindow.ui
        adapter.h
        Resources.qrc
)


add_executable(qtpassgen ${PROJECT_SOURCES})

if (WIN32)
    target_link_libraries(qtpassgen PRIVATE Qt6::Widgets ntdll advapi32 user32 ws2_32 userenv ${CMAKE_CURRENT_SOURCE_DIR}/../target/release/cpp_adapter.lib)
endif (WIN32)

if (UNIX)
    target_link_libraries(qtpassgen PRIVATE Qt6::Widgets dl ${CMAKE_CURRENT_SOURCE_DIR}/../target/release/libcpp_adapter.a)
endif (UNIX)


