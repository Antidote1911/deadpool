QT       += core gui

greaterThan(QT_MAJOR_VERSION, 4): QT += widgets

CONFIG += c++17

# You can make your code fail to compile if it uses deprecated APIs.
# In order to do so, uncomment the following line.
#DEFINES += QT_DISABLE_DEPRECATED_BEFORE=0x060000    # disables all the APIs deprecated before Qt 6.0.0

unix: LIBS += -L$$PWD/../target/release/ -lcpp_adapter
unix: PRE_TARGETDEPS += $$PWD/../target/release/libcpp_adapter.a
unix: LIBS += -ldl

SOURCES += \
    main.cpp \
    mainwindow.cpp

HEADERS += \
    adapter.h \
    mainwindow.h

FORMS += \
    mainwindow.ui

DISTFILES += \
    assets/lock_icon.png \
    assets/theme_images/checkbox.png \
    assets/theme_images/down_arrow.png \
    assets/theme_images/handle.png \
    assets/fonts/TitilliumWeb-Regular.ttf \
    stylesheets/main.qss

RESOURCES += \
    Resources.qrc

RC_ICONS = assets/lock_icon.ico

# Default rules for deployment.
qnx: target.path = /tmp/$${TARGET}/bin
else: unix:!android: target.path = /opt/$${TARGET}/bin
!isEmpty(target.path): INSTALLS += target
