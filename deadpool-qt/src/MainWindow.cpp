#include "MainWindow.h"
#include "ui_MainWindow.h"
#include "deadpool_ffi.h"

#include <QApplication>
#include <QClipboard>
#include <array>

MainWindow::MainWindow(QWidget* parent)
    : QWidget(parent), ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    // Charset toggles
    for (auto* btn : {ui->btnUppercase, ui->btnLowercase, ui->btnDigits,
                      ui->btnLogograms, ui->btnMath, ui->btnPunctuation,
                      ui->btnQuotes,    ui->btnDashes,  ui->btnBraces}) {
        connect(btn, &QPushButton::toggled, this, [this](bool) { regenerate(); });
    }

    // Length slider ↔ spinbox
    connect(ui->lengthSlider, &QSlider::valueChanged, this, [this](int v) {
        ui->lengthSpin->blockSignals(true);
        ui->lengthSpin->setValue(v);
        ui->lengthSpin->blockSignals(false);
        regenerate();
    });
    connect(ui->lengthSpin, &QSpinBox::valueChanged, this, [this](int v) {
        ui->lengthSlider->blockSignals(true);
        ui->lengthSlider->setValue(v);
        ui->lengthSlider->blockSignals(false);
        regenerate();
    });

    // Count slider ↔ spinbox
    connect(ui->countSlider, &QSlider::valueChanged, this, [this](int v) {
        ui->countSpin->blockSignals(true);
        ui->countSpin->setValue(v);
        ui->countSpin->blockSignals(false);
        regenerate();
    });
    connect(ui->countSpin, &QSpinBox::valueChanged, this, [this](int v) {
        ui->countSlider->blockSignals(true);
        ui->countSlider->setValue(v);
        ui->countSlider->blockSignals(false);
        regenerate();
    });

    connect(ui->includeEdit, &QLineEdit::textChanged, this, [this](const QString&) { regenerate(); });
    connect(ui->excludeEdit, &QLineEdit::textChanged, this, [this](const QString&) { regenerate(); });

    connect(ui->btnGenerate,  &QPushButton::clicked, this, &MainWindow::onGenerate);
    connect(ui->btnCopyAll,   &QPushButton::clicked, this, &MainWindow::onCopyAll);
    connect(ui->btnLookalike, &QPushButton::clicked, this, &MainWindow::onAddLookalike);
    connect(ui->passwordList, &QListWidget::itemClicked, this, &MainWindow::onPasswordClicked);

    regenerate();
}

MainWindow::~MainWindow() { delete ui; }

void MainWindow::onGenerate() { regenerate(); }

void MainWindow::onCopyAll() {
    QStringList all;
    for (int i = 0; i < ui->passwordList->count(); ++i)
        all << ui->passwordList->item(i)->data(Qt::UserRole).toString();
    QApplication::clipboard()->setText(all.join('\n'));
    m_allCopied = true;

    const QColor highlight = qApp->palette().color(QPalette::Highlight);
    for (int i = 0; i < ui->passwordList->count(); ++i)
        ui->passwordList->item(i)->setForeground(highlight);
}

void MainWindow::onPasswordClicked(QListWidgetItem* item) {
    QApplication::clipboard()->setText(item->data(Qt::UserRole).toString());
    m_copiedPassword = item->data(Qt::UserRole).toString();
    m_allCopied = false;

    const QColor normal    = qApp->palette().color(QPalette::Text);
    const QColor highlight = qApp->palette().color(QPalette::Highlight);
    for (int i = 0; i < ui->passwordList->count(); ++i)
        ui->passwordList->item(i)->setForeground(normal);
    item->setForeground(highlight);
}

void MainWindow::onAddLookalike() {
    QString current = ui->excludeEdit->text();
    for (const QChar ch : QStringLiteral("lBGIO0168o|.")) {
        if (!current.contains(ch))
            current += ch;
    }
    ui->excludeEdit->setText(current);
}

bool MainWindow::hasAnySet() const {
    return ui->btnUppercase->isChecked()    || ui->btnLowercase->isChecked()
        || ui->btnDigits->isChecked()        || ui->btnBraces->isChecked()
        || ui->btnPunctuation->isChecked()   || ui->btnQuotes->isChecked()
        || ui->btnDashes->isChecked()        || ui->btnMath->isChecked()
        || ui->btnLogograms->isChecked()     || !ui->includeEdit->text().isEmpty();
}

std::size_t MainWindow::minLength() const {
    const std::array<bool, 9> sets = {
        ui->btnUppercase->isChecked(),   ui->btnLowercase->isChecked(),
        ui->btnDigits->isChecked(),      ui->btnBraces->isChecked(),
        ui->btnPunctuation->isChecked(), ui->btnQuotes->isChecked(),
        ui->btnDashes->isChecked(),      ui->btnMath->isChecked(),
        ui->btnLogograms->isChecked()
    };
    std::size_t n = 0;
    for (bool b : sets) if (b) ++n;
    if (!ui->includeEdit->text().isEmpty()) ++n;
    return n;
}

void MainWindow::setStatus(const QString& text, bool error) {
    QPalette p = ui->statusLabel->palette();
    p.setColor(QPalette::WindowText,
               error ? qApp->palette().color(QPalette::LinkVisited)
                     : qApp->palette().color(QPalette::PlaceholderText));
    ui->statusLabel->setPalette(p);
    ui->statusLabel->setText(text);
}

void MainWindow::regenerate() {
    ui->passwordList->clear();
    m_copiedPassword.clear();
    m_allCopied = false;

    if (!hasAnySet()) {
        setStatus(QStringLiteral("⚠  Select at least one character group."), true);
        ui->btnGenerate->setEnabled(false);
        ui->btnCopyAll->setEnabled(false);
        return;
    }
    ui->btnGenerate->setEnabled(true);

    const auto minLen = static_cast<int>(minLength());
    if (ui->lengthSlider->value() < minLen) {
        ui->lengthSlider->setValue(minLen);
        return;
    }

    DeadpoolPool* pool = deadpool_pool_new();

    if (ui->btnUppercase->isChecked())   deadpool_extend_uppercase(pool);
    if (ui->btnLowercase->isChecked())   deadpool_extend_lowercase(pool);
    if (ui->btnDigits->isChecked())      deadpool_extend_digits(pool);
    if (ui->btnBraces->isChecked())      deadpool_extend_braces(pool);
    if (ui->btnPunctuation->isChecked()) deadpool_extend_punctuation(pool);
    if (ui->btnQuotes->isChecked())      deadpool_extend_quotes(pool);
    if (ui->btnDashes->isChecked())      deadpool_extend_dashes(pool);
    if (ui->btnMath->isChecked())        deadpool_extend_math(pool);
    if (ui->btnLogograms->isChecked())   deadpool_extend_logograms(pool);

    const QByteArray include = ui->includeEdit->text().toUtf8();
    if (!include.isEmpty()) {
        if (deadpool_extend_from_string(pool, include.constData()) != 0) {
            setStatus(QStringLiteral("⚠  ") + QString::fromUtf8(deadpool_last_error()), true);
            deadpool_pool_free(pool);
            ui->btnCopyAll->setEnabled(false);
            return;
        }
    }

    const QByteArray exclude = ui->excludeEdit->text().toUtf8();
    if (!exclude.isEmpty())
        deadpool_exclude_chars(pool, exclude.constData());

    constexpr std::size_t kBufSize = 1024;
    char buf[kBufSize];
    const int  count  = ui->countSpin->value();
    const auto length = static_cast<std::size_t>(ui->lengthSpin->value());
    bool anyError = false;

    for (int i = 0; i < count; ++i) {
        if (deadpool_generate(pool, length, buf, kBufSize) == 0) {
            const QString pw = QString::fromUtf8(buf);
            auto* item = new QListWidgetItem(pw);
            item->setData(Qt::UserRole, pw);
            ui->passwordList->addItem(item);
        } else {
            setStatus(QStringLiteral("⚠  ") + QString::fromUtf8(deadpool_last_error()), true);
            anyError = true;
            break;
        }
    }

    deadpool_pool_free(pool);

    if (!anyError) {
        setStatus(QStringLiteral("ℹ  Click a password to copy it"), false);
        ui->btnCopyAll->setEnabled(ui->passwordList->count() > 0);
    } else {
        ui->btnCopyAll->setEnabled(false);
    }
}
