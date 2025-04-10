#include "mainwindow.h"
#include <QLocale>
#include <QApplication>
#include <QFile>
#include <QFontDatabase>

int main(int argc, char *argv[])
{
    QApplication a(argc, argv);

    MainWindow w;
    w.setFixedSize(QSize(530, 320));
    // Load the application style
    QFile styleFile( ":/stylesheets/main.qss" );
    styleFile.open( QFile::ReadOnly );
    // Apply the stylesheet
    a.setStyleSheet(QString(styleFile.readAll()));
    QFontDatabase::addApplicationFont(":/assets/fonts/TitilliumWeb-Regular.ttf");
    w.show();
    return a.exec();
}
