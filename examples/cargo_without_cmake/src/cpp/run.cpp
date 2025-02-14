// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_cargo_cpp_includes
#include <QtGui/QGuiApplication>
#include <QtQml/QQmlApplicationEngine>

#include "cxx-qt-gen/my_object.cxxqt.h"

// Include the C++ code generated by rcc from the .qrc file
#include "qml.qrc.cpp"
// ANCHOR_END: book_cargo_cpp_includes

// ANCHOR: book_cargo_run_cpp
// extern "C" is important for the linker to be able to link this
// function with Rust code.
extern "C" int
run_cpp(int argc, char* argv[])
{
  // ANCHOR_END: book_cargo_run_cpp

  // ANCHOR: book_cargo_init_qrc
  // Normally in a C++ program, global variables for the Qt Resource System are
  // initialized before the C++ main function runs. However, when building a
  // Rust executable with Cargo, the Qt Resource System needs to be initialized
  // manually.
  // https://doc.qt.io/qt-6/resources.html#explicit-loading-and-unloading-of-embedded-resources
  qInitResources();
  // ANCHOR_END: book_cargo_init_qrc

  // ANCHOR: book_cargo_run_qml
  QGuiApplication app(argc, argv);

  QQmlApplicationEngine engine;

  const QUrl url(QStringLiteral("qrc:/main.qml"));
  QObject::connect(
    &engine,
    &QQmlApplicationEngine::objectCreated,
    &app,
    [url](QObject* obj, const QUrl& objUrl) {
      if (!obj && url == objUrl)
        QCoreApplication::exit(-1);
    },
    Qt::QueuedConnection);

  qmlRegisterType<MyObject>("com.kdab.cxx_qt.demo", 1, 0, "MyObject");

  engine.load(url);

  return app.exec();
}
// ANCHOR_END: book_cargo_run_qml
