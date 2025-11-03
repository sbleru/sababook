#!/bin/bash -xe

# Docker環境でSaBaブラウザを実行するスクリプト

# 引数チェック
if [ $# -eq 0 ]; then
    echo "Usage: $0 <chapter_directory>"
    echo "Example: $0 ch6/saba"
    exit 1
fi

CHAPTER_DIR=$1
HOME_PATH="/workspace/$CHAPTER_DIR"
TARGET_PATH="$HOME_PATH/build"
OS_PATH="$TARGET_PATH/wasabi"
APP_NAME="saba"
MAKEFILE_PATH="$HOME_PATH/Makefile"

echo "Running SaBa browser in chapter: $CHAPTER_DIR"

# チャプターディレクトリの存在確認
if [ ! -d "$HOME_PATH" ]; then
    echo "Error: Chapter directory $HOME_PATH does not exist"
    exit 1
fi

# チャプターディレクトリに移動
cd "$HOME_PATH"

# buildディレクトリを作成する
if [ -d "$TARGET_PATH" ]; then
    echo "$TARGET_PATH exists"
else
    echo "$TARGET_PATH doesn't exist"
    mkdir "$TARGET_PATH"
fi

# WasabiOSをダウンロードする
if [ -d "$OS_PATH" ]; then
    echo "$OS_PATH exists"
    cd "$OS_PATH"
    
    # Docker環境対応のためHTTPS URLに設定
    echo "Setting remote URL to HTTPS..."
    git remote set-url origin https://github.com/hikalium/wasabi.git
    
    echo "pulling new changes..."
    git pull origin for_saba
else
    echo "$OS_PATH doesn't exist"
    echo "cloning wasabi project..."
    cd "$TARGET_PATH"
    git clone --branch for_saba https://github.com/hikalium/wasabi.git
fi

# アプリケーションのトップディレクトリに移動する
cd "$HOME_PATH"

# Makefileをダウンロードする
if [ ! -f "$MAKEFILE_PATH" ]; then
    echo "downloading Makefile..."
    MAKEFILE_URL="https://raw.githubusercontent.com/hikalium/wasabi/for_saba/external_app_template/Makefile"
    
    # curlを利用する（Dockerイメージにはcurlがインストール済み）
    curl -O "$MAKEFILE_URL"
fi

# ビルドと実行
echo "Building application..."
make build

echo "Starting QEMU with GUI display..."
# Docker環境ではDISPLAY=:1を使用
export DISPLAY=:1
"$OS_PATH/scripts/run_with_app.sh" "./target/x86_64-unknown-none/release/$APP_NAME"
