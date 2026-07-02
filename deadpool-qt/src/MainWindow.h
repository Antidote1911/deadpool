#pragma once
#include <QListWidgetItem>
#include <QWidget>

QT_BEGIN_NAMESPACE
namespace Ui { class MainWindow; }
QT_END_NAMESPACE

class MainWindow final : public QWidget {
    Q_OBJECT

public:
    explicit MainWindow(QWidget* parent = nullptr);
    ~MainWindow() override;

private slots:
    void onGenerate();
    void onCopyAll();
    void onPasswordClicked(QListWidgetItem* item);
    void onAddLookalike();

private:
    void regenerate();
    void setStatus(const QString& text, bool error);
    [[nodiscard]] bool hasAnySet() const;
    [[nodiscard]] std::size_t minLength() const;

    Ui::MainWindow* ui;
    QString m_copiedPassword;
    bool    m_allCopied{false};
};
