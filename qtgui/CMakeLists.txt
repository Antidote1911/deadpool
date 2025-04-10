cmake_minimum_required(VERSION 3.19)

project(qtpassgen LANGUAGES CXX)

set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 6.5 REQUIRED COMPONENTS Widgets)

set(PROJECT_SOURCES
        main.cpp
        mainwindow.cpp
        mainwindow.h
        mainwindow.ui
        adapter.h
        Resources.qrc
)


add_executable(qtpassgen ${PROJECT_SOURCES})
target_link_libraries(qtpassgen PRIVATE Qt6::Widgets dl ${CMAKE_CURRENT_SOURCE_DIR}/../target/release/libcpp_adapter.a)

