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

## 環境構築

Rustをインストールする

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

QEMUをインストールする

```sh
brew install qemu
```

## 実行

GUIを立ち上げるプロジェクトの場合

```sh
cd ch6/saba
export DISPLAY=0 && ./run_on_wasabi.sh
```
