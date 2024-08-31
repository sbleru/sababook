#!/bin/bash -xe

HOME_PATH=$PWD
TARGET_PATH=$PWD"/build"
OS_PATH=$TARGET_PATH"/wasabi"
APP_NAME="saba"
APP_PATH=$OS_PATH"/app/"$APP_NAME

# build ディレクトリを作成する
if [ -d $TARGET_PATH ]
then
  echo $TARGET_PATH" exists"
else
  echo $TARGET_PATH" doesn't exist"
  mkdir $TARGET_PATH
fi

# WasabiOSをダウンロードする (https://github.com/hikalium/wasabi)
# もしスクリプトが失敗する場合は、`rm -rf build/wasabi`などでダウンロードしたOSを削除する必要がある
if [ -d $OS_PATH ]
then
  echo $OS_PATH" exists"
  echo "pulling new changes..."
  cd $OS_PATH
  git fetch --all
  git branch backup
  git reset --hard origin/main
  git branch -D backup
else
  echo $OS_PATH" doesn't exist"
  echo "cloning wasabi project..."
  cd $TARGET_PATH
  git clone git@github.com:hikalium/wasabi.git
fi

cd $HOME_PATH

# build/wasabi/app/saba/ディレクトリを作成する
if [ -d $APP_PATH ]
then
  echo $APP_PATH" exists"
else
  echo $APP_PATH" doesn't exist"
  mkdir $APP_PATH
fi

# アプリケーションのコードをbuild/wasabi/app/saba/ディレクトリ以下にコピーする
echo "copying the project to wasabi OS..."
cp -R `ls -A ./ | grep -v "target" | grep -v ".git" | grep -v "build"` $APP_PATH

cd $OS_PATH

# Cargo.tomlの[workspace]にメンバーを追加する
mv Cargo.toml Cargo.toml.original
if [ $(grep -c "app/$APP_NAME" Cargo.toml.original) -eq 1 ]
then
  echo "$APP_PATH already exists in Cargo.toml"
  mv Cargo.toml.original Cargo.toml
else
  sed "s/^members = \[/members = \[\n    \"app\/$APP_NAME\",/" Cargo.toml.original >| Cargo.toml
fi
rm Cargo.toml.original

make run

cd $HOME_PATH
