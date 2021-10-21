# reclick | リクリック

Recorder and replayer of mouse clicks.

マウスクリックをレコードしリプレイするツール。

## Features | 機能

Only works for left clicks. Drag and drop may work but untested.

左クリックだけ。ドラッグ&ドロップはテストしてなかったけど、できるかも。

## Installation | インストール

    cargo install reclick

## macOS Permission | macOS 許可

This needs "Input Monitoring" permission and "Accessibility" permission on macOS.

"Input Monitoring" so you can start recording with a keypress outside of its window.

"Accessibility" so it can replay the recorded mouse clicks.

Please grant these to "Terminal.app" in "System Preferences" > "Security & Privacy" > "Privacy".

## Build

### Build for native

    cargo build --release

### Cross build for windows on macOS

Install dependencies:

    brew install mingw-w64
    rustup add target x86_64-pc-windows-gnu

Build:

    cargo build --release --target x86_64-pc-windows-gnu