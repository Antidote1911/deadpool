#include "MainWindow.h"
#include "deadpool_ffi.h"

#include <QApplication>
#include <QIcon>
#include <QStyleHints>

int main(int argc, char* argv[]) {
    QApplication app(argc, argv);
    app.setApplicationName(QStringLiteral("deadpool"));
    app.setApplicationVersion(QString::fromUtf8(deadpool_version()));
    app.setOrganizationName(QStringLiteral("Antidote1911"));

    // Native dark colour scheme — no custom stylesheet needed
    app.styleHints()->setColorScheme(Qt::ColorScheme::Dark);

    app.setWindowIcon(QIcon(QStringLiteral(":/icon.png")));

    MainWindow window;
    window.show();

    return app.exec();
}
