# 本書を読む前の準備

[作って学ぶ　ブラウザのしくみ　書き起こし](https://chatgpt.com/c/67abd434-e58c-8011-920a-439ed15ac365)

## 環境構築

本書のサンプルプログラムは macOS と Ubuntu でテストされており、macOS や Ubuntu や Debian GNU/Linux などの Linux ディストリビューション上で開発することを想定しています。Windows では、WSL (Windows Subsystem for Linux) などを使用して Windows 上で仮想的に Linux 環境を作ることで対応が可能です。

### Rust のインストール

本書のサンプルプログラムは、プログラミング言語の一つである Rust[1](https://chatgpt.com/c/67abd434-e58c-8011-920a-439ed15ac365#user-content-fn-1) で書かれています。Rust は、複数のツールを使ってプログラムの管理をします。プログラムをコンパイルしたり実行したりするために必要な一連のツール群をツールチェインと呼びます。

ツールチェインをインストールするために、ターミナルを開いて以下のコマンドを実行してください。このコマンドは公式ページ[2](https://chatgpt.com/c/67abd434-e58c-8011-920a-439ed15ac365#user-content-fn-2) に記載されているものと同等です。

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

ツールチェインをインストールすると、以下のコマンドが使用できるようになっているはずです。

- **rustup**
    
    Rust のツールチェイン管理ツール。Rust のツールチェインのインストール、アップデート、管理に使用する
    
- **rustc**
    
    Rust のコンパイラ。Rust のソースコードをバイナリコードに変換する
    
- **cargo**
    
    Rust のビルドツール。Rust のプロジェクトをビルド、テスト、デプロイするために使用する
    

### 本書で使用している Rust のバージョン

本書のサンプルプログラムは、関連書『[作って学ぶ] OS のしくみ』で説明されているゼロから作成した WasabiOS[3](https://chatgpt.com/c/67abd434-e58c-8011-920a-439ed15ac365#user-content-fn-3) という自作 OS のアプリケーションとして動作します。その特性上、nightly というバージョンのツールチェインを使用する必要があります。Rust には、nightly・beta・stable という 3 段階のリリースサイクルを経てユーザーのもとに届きます。デフォルトで使用されているのが stable バージョンで、その名のとおり、一番安定した機能を含んでいます。対して nightly は、最も実験的でかつ最新の機能を含むバージョンです。nightly は毎日リリースされるのに対し、stable は 6 週間ごとにリリースされます。本書執筆時点 (2024 年 7 月) で、OS 開発のためは nightly でしか採用されていない機能を使う必要があるので、OS で使用しているツールチェインに合わせて本書でも nightly を使用しています。

ツールチェインのバージョンを指定できる `rust-toolchain.toml` をプロジェクトのトップディレクトリに追加しましょう。

```toml
rust-toolchain.toml

[toolchain]
channel = "nightly-2024-01-01"
components = [ "rustfmt", "rust-src" ]
targets = [ "x86_64-unknown-linux-gnu" ]
profile = "default"

```

現在インストールされている Rust コンパイラのバージョンを `rustup show` コマンドで確認すると、nightly-2024-01-01 の日付のものになっているはずです。

```bash
$ rustup show
(省略)
active toolchain
----------------
nightly-2024-01-01-aarch64-apple-darwin (overridden by '/Users/.../rust-toolchain.toml')

```

---

## QEMU のインストール

QEMU とは、オープンソースのエミュレータです。エミュレータとは、あるコンピュータシステムが別のコンピュータシステムの機能を模倣するソフトウェアまたはハードウェアです。普通、アプリケーションを開発しているときにエミュレータは必要ありません。しかし本書のブラウザは、WasabiOS の上で動かすためにエミュレータが必要です。

QEMU をインストールするために、Mac を使用している方は以下を実行してください。

```bash
$ brew install qemu

```

Debian GNU/Linux や Ubuntu を使っている方は、以下を実行してください。

```bash
$ apt install qemu-system

```

ほかの環境で開発している方は、公式のページ[4](https://chatgpt.com/c/67abd434-e58c-8011-920a-439ed15ac365#user-content-fn-4) を参考にダウンロードとインストールをしてください。

---

## Git のインストール

Git はプログラムのバージョン管理を行うツールです。ソースコードをダウンロードするときに Git を使用するので、もし今まで使用したことがなければインストールしてください。

Git をインストールするために、Mac を使用している方は以下を実行してください。

```bash
$ brew install git

```

Debian GNU/Linux や Ubuntu を使っている方は、以下を実行してください。

```bash
$ apt install git-all

```

ほかの環境で開発している方は、公式のページ[5](https://chatgpt.com/c/67abd434-e58c-8011-920a-439ed15ac365#user-content-fn-5) を参考にダウンロードとインストールをしてください。

---

## サンプルプログラム

本書で解説して実装するブラウザのプログラムは、2 つの GitHub リポジトリに掲載されています。サンプルブラウザアプリケーション (Sample Browser Application) を略して SaBA という名前です。もし本書を参考にして自分でプログラムを書いているときに思ったように動かなければ、これらのリポジトリのコードと見比べてみてください。

- https://github.com/d0iasm/saba
    
    最新の変更/修正を含むリポジトリ。本書で書かれていること以上の実装を含む
    
- https://github.com/d0iasm/sababook
    
    本書とまったく同じコードのリポジトリ。章ごとでディレクトリが分かれている
    

### SaBA ブラウザの構成

SaBA の大まかなディレクトリ構造は以下のようになっています。build/ ディレクトリや便利スクリプトなどは省略しています。

```bash
saba $ tree -L 2
.
├── Cargo.toml
├── README.md
├── saba_core
│   ├── Cargo.toml
│   └── src
├── src
│   └── main.rs
├── net
│   ├── std
│   └── wasabi
└── ui
    ├── cui
    └── wasabi

```

一番のメインとなる実装は `saba_core/` ディレクトリ以下に存在します。`src/` ディレクトリは `main` 関数を含むアプリケーションのエントリポイントになります。それ以外の `net/`, `ui/` ディレクトリは、アプリケーションを動かす OS によって実装を変える必要があるため、ディレクトリが細分化されています。ただし、本書では WasabiOS 上で動かす実装のみを紹介します。

- **saba_core**
    
    HTML, CSS, JavaScript を解析してページをレンダリングする機能の実装。外部クレートへの依存関係を持たない。第2章, 第4章, 第5章, 第7章で実装
    
- **src**
    
    アプリケーションのエントリポイントとなるメイン関数の実装。各章で少しずつ実装
    
- **net**
    
    ネットワークに関する機能の実装。第3章で実装
    
- **ui**
    
    ユーザーインタフェースに関する機能の実装。第6章で実装
    

---

## WasabiOS の構成

SaBA を動かす OS は、関連書で解説されている WasabiOS[6](https://chatgpt.com/c/67abd434-e58c-8011-920a-439ed15ac365#user-content-fn-6) を使用します。さらに深掘りして、ネットワークの根幹や OS がどのようにリソースを管理しているかまで理解したい場合は、こちらのリポジトリと関連書『[作って学ぶ] OS のしくみ』も参考にしてください。

WasabiOS 上でアプリケーションを開発する際に特に重要なのは WasabiOS リポジトリの `noli/` ディレクトリです。これは OS とアプリケーションをつなぐライブラリ群で、文字や図形の描画などの機能をアプリケーションに提供しています。

---

## アプリケーションを WasabiOS で動かす

アプリケーションを WasabiOS で動かすためには、cargo ビルド コマンドでビルドしたアプリケーションのバイナリを、WasabiOS が提供する `run_with_app.sh` というスクリプトを使用して走らせる必要があります。これらを自動的に行ってくれる便利なシェルスクリプトを用意したので、以下のスクリプトを自分のプロジェクトに追加してください。または、d0iasm/saba の `run_on_wasabi.sh` からコピーすることもできます。

```bash
run_on_wasabi.sh
#!/bin/bash -xe

HOME_PATH=$PWD
TARGET_PATH="$PWD"/build
OS_PATH="$TARGET_PATH"/wasabi
# アプリケーションの名前が saba と異なるとき、次の行を変更する
APP_NAME="saba"
MAKEFILE_PATH="$HOME_PATH"/Makefile

# build ディレクトリを作成する
if [ -d $TARGET_PATH ]
then
  echo "$TARGET_PATH" exists
else
  echo "$TARGET_PATH" doesn't exist
  mkdir $TARGET_PATH
fi

# WasabiOS をダウンロードする (https://github.com/hikalium/wasabi)
# もしスクリプトが失敗する場合は、`rm -rf build/wasabi` などで
# ダウンロードした OS を削除する必要がある
if [ -d $OS_PATH ]
then
  echo "$OS_PATH" exists
  echo "pulling new changes..."
  cd $OS_PATH
  git pull origin for_saba
else
  echo "$OS_PATH" doesn't exist
  echo "cloning wasabi project..."
  cd $TARGET_PATH
  git clone --branch for_saba git@github.com:hikalium/wasabi.git
fi

# アプリケーションのトップディレクトリに移動する
cd $HOME_PATH

# Makefile をダウンロードする
if [ ! -f $MAKEFILE_PATH ]; then
  echo "downloading Makefile..."
  wget https://raw.githubusercontent.com/hikalium/wasabi/main/external_app_template/Makefile
fi

make build
$OS_PATH/scripts/run_with_app.sh ./target/x86_64-unknown-none/release/$APP_NAME

```

シェルスクリプトをトップディレクトリに追加したら、`chmod` コマンドを使用してシェルスクリプトに実行権限を与えましょう。

```bash
$ chmod +x run_on_wasabi.sh

```

アプリケーションのトップディレクトリで `run_on_wasabi.sh` のスクリプトを走らせると、アプリケーションが WasabiOS の上で開始します。もしアプリケーションの名前を独自のものにした場合は、スクリプトの `APP_NAME` を変更してください。

また、もしスクリプトが途中で失敗したら、`rm -rf build` などによりダウンロードした WasabiOS のソースコードを削除してみてください。

---

## プロジェクトの作成

プロジェクトを作成してみましょう。cargo コマンドを使用することによって簡単に新しいプロジェクトを作成できます。`cargo new` コマンドとそれに続いてプロジェクト名を入力することで新しいディレクトリを作成します。ディレクトリの配下には `Cargo.toml` と `src` ディレクトリが自動的に作成されます。

```bash
$ cargo new saba

```

`Cargo.toml` はプロジェクトの設定を管理するための設定ファイルです。ライブラリの依存関係などをここに書きます。WasabiOS とやりとりするための noli ライブラリを使えるように `Cargo.toml` を書き換えてみましょう。

```toml
Cargo.toml

[package]
name = "saba"
version = "0.1.0"
edition = "2021"

[dependencies]
noli = { git = "https://github.com/hikalium/wasabi.git", branch = "for_saba" }

```

`src` ディレクトリ以下の `main.rs` を変更してみましょう。WasabiOS は、スタンダードライブラリに依存せずに書かれています。スタンダードライブラリとは、Rust では `std` によってインポートできるライブラリ群のことです。OS の制約上、アプリケーションも同じくスタンダードライブラリに依存せずに書く必要があります。よって、ファイルの最初に `#![no_std]` と書いてください。

noli ライブラリの API を使用するために、`use noli::prelude::*;` も必要です。これで、文字を出力したり図形を描画したりできます。

```rust
src/main.rs

#![no_std]
#![cfg_attr(not(target_os = "linux"), no_main)]

use noli::prelude::*;

fn main() {
    Api::write_string("Hello World\n");
    println!("Hello from println!");
    Api::exit(42);
}

entry_point!(main);

```

`run_on_wasabi.sh` スクリプトを使用して OS 上でアプリケーションを動かしてみましょう。

```bash
$ ./run_on_wasabi.sh

```

スクリプトを走らせると、QEMU のアプリケーションが開始します (図 0-1)。その画面上またはターミナル上でアプリケーションの名前 (saba) を入力して Enter キーを押すと、そのアプリケーションが開始します。

### 図 0-1 QEMU のスタート直後の画面

*(画像：紙の船のようなロゴが表示された QEMU ウィンドウ)*

WasabiOS は、QEMU の画面の下部にログが出力します。上記の “Hello World!” の文字列を出力するアプリケーションを開始すると、QEMU とターミナルのどちらでもログの出力が確認できます (図 0-2)。

### 図 0-2 ログの出力結果

```
Hello World
Hello from println!
[INFO] os/src/cmd.rs:117: Ok(42)

```

---

## 本書のコードの読み方

第 2 章から実装していくブラウザのコードは、本書で以下のように書かれています。

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    (省略)
}
/// https://262.ecma-international.org/#sec-identifier-names
Identifier(String),
/// https://262.ecma-international.org/#sec-keywords-and-reserved-words
Keyword(String),
/// https://262.ecma-international.org/#sec-literals-string-literals
StringLiteral(String),
}

```

新しく実装する箇所は太字で書かれているので、書籍を読みながら実装していく方は太字の箇所を自分のプログラムに随時追加してくだい。もしコード中に太字がまったくない場合は、すべてのコードを追加する必要があるという意味です。また、「(省略)」と書かれている部分はすでに実装を紹介した箇所です。

コード中に時折出てくる URL は、その実装に対応する仕様書への URL を表します。もし仕様書ではどのように書かれているのか気になる方は自分で確かめてみてください。

---

## 注意事項

本書で解説・実装するブラウザのアプリケーションは自作 OS 上で動いているため、さまざまな制約があります。たとえば、アプリケーションが使用できるメモリには限りがあります。なので、もしページ遷移を繰り返すと、アウトオブメモリ、つまり必要なメモリ容量をこれ以上確保できず実行が中断してしまうなどの問題があります。

また、OS にはアプリケーションを中断する機能がありません。もしブラウザのアプリケーションを終了したいときは、QEMU のアプリケーション自体を閉じて、OS の実行自体を終了させてください。

さらに、アクセスできる Web サイトは HTTP から始まるページのみです。通信が暗号化されている HTTPS から始まるページにはアクセスできないことに注意してください。

ブラウザも OS もとても巨大なプログラムで、かつ、さまざまな使い方が存在します。本書で明示的に解説されている使い方以外は、バグを含んでいる可能性が大いにあることにご注意ください。もし明らかなバグを見つけた場合は、saba リポジトリの issue に報告していただけるとうれしいです。

---

注 1 https://www.rust-lang.org/

*(本文に戻る)*

注 2 https://rustup.rs/

*(本文に戻る)*

注 3 https://github.com/hikalium/wasabi

*(本文に戻る)*

注 4 https://www.qemu.org/download/

*(本文に戻る)*

注 5 https://git-scm.com/downloads

*(本文に戻る)*

注 6 https://github.com/hikalium/wasabi

*(本文に戻る)*

注 7 https://github.com/d0iasm/saba/blob/main/run_on_wasabi.sh

*(本文に戻る)*

**Footnotes**
1. Rust 公式サイト: https://www.rust-lang.org/ [↩](https://chatgpt.com/c/67abd434-e58c-8011-920a-439ed15ac365#user-content-fnref-1)
2. Rust インストールガイド: https://www.rust-lang.org/tools/install [↩](https://chatgpt.com/c/67abd434-e58c-8011-920a-439ed15ac365#user-content-fnref-2)
3. WasabiOS リポジトリ: https://github.com/hikalium/wasabi [↩](https://chatgpt.com/c/67abd434-e58c-8011-920a-439ed15ac365#user-content-fnref-3)
4. QEMU 公式サイト: https://www.qemu.org/ [↩](https://chatgpt.com/c/67abd434-e58c-8011-920a-439ed15ac365#user-content-fnref-4)
5. Git 公式サイト: https://git-scm.com/ [↩](https://chatgpt.com/c/67abd434-e58c-8011-920a-439ed15ac365#user-content-fnref-5)
6. WasabiOS の詳細: https://github.com/hikalium/wasabi (関連書『[作って学ぶ] OS のしくみ』) [↩](https://chatgpt.com/c/67abd434-e58c-8011-920a-439ed15ac365#user-content-fnref-6)

1. 1.
    
    Rust 公式サイト: https://www.rust-lang.org/ [↩](https://chatgpt.com/c/67abd434-e58c-8011-920a-439ed15ac365#user-content-fnref-1)
    
2. 2.
    
    Rust インストールガイド: https://www.rust-lang.org/tools/install [↩](https://chatgpt.com/c/67abd434-e58c-8011-920a-439ed15ac365#user-content-fnref-2)
    
3. 3.
    
    WasabiOS リポジトリ: https://github.com/hikalium/wasabi [↩](https://chatgpt.com/c/67abd434-e58c-8011-920a-439ed15ac365#user-content-fnref-3)
    
4. 4.
    
    QEMU 公式サイト: https://www.qemu.org/ [↩](https://chatgpt.com/c/67abd434-e58c-8011-920a-439ed15ac365#user-content-fnref-4)
    
5. 5.
    
    Git 公式サイト: https://git-scm.com/ [↩](https://chatgpt.com/c/67abd434-e58c-8011-920a-439ed15ac365#user-content-fnref-5)
    
6. 6.
    
    WasabiOS の詳細: https://github.com/hikalium/wasabi (関連書『[作って学ぶ] OS のしくみ』) [↩](https://chatgpt.com/c/67abd434-e58c-8011-920a-439ed15ac365#user-content-fnref-6)