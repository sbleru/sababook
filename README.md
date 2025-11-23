# SaBA (Sample Browser Application)

本リポジトリは書籍『[［作って学ぶ］ブラウザのしくみ──HTTP、HTML、CSS、JavaScriptの裏側](https://amzn.asia/d/dcEmU3E)』で解説されているソースコードです。

本リポジトリの各ディレクトリはそれぞれの章に対応しています。

- [ch0/](./ch0/saba) — 『本書を読む前の準備』
- [ch2/](./ch2/saba) — 『第２章　URLを分解する』
- [ch3/](./ch3/saba) — 『第３章　HTTPを実装する』
- [ch4/](./ch4/saba) — 『第４章　HTMLを解析する』
- [ch5/](./ch5/saba) — 『第５章　CSSで装飾する』
- [ch6/](./ch6/saba) — 『第６章　GUIを実装する』
- [ch7/](./ch7/saba) — 『第７章　JavaScriptを動かす』

書籍で解説されていない実験的なコードや最新の変更は[saba](https://github.com/d0iasm/saba)リポジトリに存在します。

もしプログラム中に問題を見つけた場合は、[Issues](https://github.com/d0iasm/sababook/issues)に報告をいただけると嬉しいです。

# Getting Started

```sh
git clone https://github.com/sbleru/sababook
cd sababook
docker compose build
docker compose up -d

# ブラウザで http://localhost:6080 にアクセスし、「接続」ボタンを押す

# コンテナ内のシェルに接続
docker compose exec saba-dev bash

# コンテナ内で各章のブラウザを実行（ここでは7章を指定）
root@xxx:/workspace# ./docker/run_on_docker.sh ch7/saba

# saba を入力

# ブラウザに戻る

python3 -m http.server 8000 -d ch7/saba & pid=$!; trap "kill $pid" INT; ./docker/run_on_docker.sh ch7/saba; kill $pid

# http://host.test:8000/test.html にアクセス
```

## 環境構築

### Docker環境（推奨）

OS環境に依存しない統一された開発環境を提供します。

```sh
# Docker Desktopをインストール後
docker compose build
docker compose up -d

# ブラウザで http://localhost:6080 にアクセス
```

詳細は [Docker環境セットアップガイド](./docs/DOCKER_SETUP.md) をご覧ください。

```sh
# コンテナを削除
docker compose down
```

### ネイティブ環境

Rustをインストールする

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

QEMUをインストールする

```sh
brew install qemu
```

## 実行

### Docker環境での実行

```sh
# コンテナ内でシェルを起動
docker compose exec saba-dev bash

# 各章のブラウザを実行
./docker/run_on_docker.sh ch6/saba
```

### ネイティブ環境での実行

GUIを立ち上げるプロジェクトの場合

```sh
cd ch6/saba
export DISPLAY=0 && ./run_on_wasabi.sh
```
