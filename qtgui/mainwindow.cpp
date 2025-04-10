#include "mainwindow.h"
#include "ui_mainwindow.h"
#include <QMessageBox>
#include <QTimer>
#include <QClipboard>
#include <QMessageBox>
#include <QRandomGenerator>
#include <QVector>

#include "adapter.h"

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);
    ui->pushNumbers->setChecked(true);
    ui->pushUpper->setChecked(true);


    ui->spinBox->setValue(ui->horizontalSlider->value());

    connect(ui->btn_generate, &QPushButton::clicked, this, &MainWindow::onGenerateClick);
    connect(ui->btn_copy, &QPushButton::clicked, this, &MainWindow::onCopyClick);
    connect(ui->optionsButtons, &QButtonGroup::buttonClicked, this, &MainWindow::onGenerateClick);
    connect(ui->horizontalSlider, &QSlider::sliderMoved, this, &MainWindow::onGenerateClick);

    connect(ui->horizontalSlider, SIGNAL(valueChanged(int)), SLOT(passwordLengthChanged(int)));
    connect(ui->spinBox, SIGNAL(valueChanged(int)), SLOT(passwordLengthChanged(int)));
    connect(ui->btnAddLookLike, &QPushButton::clicked, this, &MainWindow::onLookClick);

    connect(ui->excludeEdit, &QLineEdit::textChanged, this, &MainWindow::onGenerateClick);
    connect(ui->alsoEdit, &QLineEdit::textChanged, this, &MainWindow::onGenerateClick);

    onGenerateClick();

}

MainWindow::~MainWindow()
{
    delete ui;
}

void MainWindow::onLookClick()
{
    ui->excludeEdit->insert("lBGIO0168o|.");

    onGenerateClick();
}

void MainWindow::passwordLengthChanged(int length)
{
    ui->spinBox->blockSignals(true);
    ui->horizontalSlider->blockSignals(true);

    ui->spinBox->setValue(length);
    ui->horizontalSlider->setValue(length);

    ui->spinBox->blockSignals(false);
    ui->horizontalSlider->blockSignals(false);

    onGenerateClick();
}

void MainWindow::onGenerateClick()
{
    const int  pwdLen = ui->horizontalSlider->sliderPosition();
    const bool lowerCase = ui->pushLower->isChecked();
    const bool upperCase = ui->pushUpper->isChecked();
    const bool numbers = ui->pushNumbers->isChecked();
    const bool braces = ui->pushBraces->isChecked();
    const bool punctuation = ui->pushPunctuation->isChecked();
    const bool quotes = ui->pushQuotes->isChecked();
    const bool dashes = ui->pushDashes->isChecked();
    const bool math = ui->pushMath->isChecked();
    const bool logograms = ui->pushLogograms->isChecked();

    auto test=ui->alsoEdit->text()!="";
    const int truegroups=lowerCase+upperCase+numbers+braces+punctuation+quotes+dashes+math+logograms+test;


    auto alsoinclide = ui->alsoEdit->text().toStdString();
    const char* also = alsoinclide.c_str();

    auto avoidchar = ui->excludeEdit->text().toStdString();
    const char* avoid = avoidchar.c_str();

    ui->statusbar->showMessage("");

    if (!lowerCase && !upperCase && !numbers && !braces && !punctuation && !quotes && !dashes && !math && !logograms)
    {
        ui->btn_generate->setDisabled(true);
        ui->horizontalSlider->setDisabled(true);
        ui->spinBox->setDisabled(true);
        ui->statusbar->showMessage("Select one or more groups");
        ui->btn_copy->setDisabled(true);

    }else{

        if (pwdLen<truegroups){

            ui->spinBox->setValue(truegroups);
        }

        ui->btn_generate->setDisabled(false);
        ui->horizontalSlider->setDisabled(false);
        ui->spinBox->setDisabled(false);
        ui->btn_copy->setDisabled(false);


        auto test = get_random(pwdLen,
                               upperCase,
                               lowerCase,
                               numbers,
                               braces,
                               punctuation,
                               quotes,
                               dashes,
                               math,
                               logograms,
                               avoid,
                               also);

        ui->passwordEdit->clear();
        ui->passwordEdit->setText(test);
    }
}


void MainWindow::onCopyClick()
{
    //copy to system clipboard only if there is a password set
    const QString pwd = ui->passwordEdit->text();
    if (pwd.length() >= 8)
    {
        QGuiApplication::clipboard()->setText(pwd);
        ui->statusbar->showMessage("Password Copied to Clipboard",2000);
    }
}

