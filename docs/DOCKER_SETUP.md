# SaBaブラウザ Docker環境セットアップガイド

## 概要

このガイドでは、SaBaブラウザをDocker環境で実行する方法を説明します。Docker化により、Windows、macOS、Linuxのどの環境でも統一された方法でブラウザ開発を行えます。

## 前提条件

### 必要なソフトウェア

- **Docker Desktop**: 各OSに対応したDocker Desktopをインストールしてください
  - [Windows用Docker Desktop](https://docs.docker.com/desktop/windows/install/)
  - [macOS用Docker Desktop](https://docs.docker.com/desktop/mac/install/)
  - [Linux用Docker Desktop](https://docs.docker.com/desktop/linux/install/)

### システム要件

- **メモリ**: 最低4GB、推奨8GB以上
- **ディスク容量**: 最低10GB以上の空き容量
- **CPU**: x64アーキテクチャ（Intel/AMD）

## セットアップ手順

### 1. リポジトリのクローン

```bash
git clone https://github.com/d0iasm/sababook.git
cd sababook
```

### 2. Dockerイメージのビルド

```bash
docker-compose build
```

初回ビルドには10-15分程度かかります。

### 3. コンテナの起動

```bash
docker-compose up -d
```

### 4. GUI環境へのアクセス

ブラウザで以下のURLにアクセスしてください：

```
http://localhost:6080
```

VNCパスワードを求められた場合は `saba123` を入力してください。

## SaBaブラウザの実行

### コンテナ内でのブラウザ実行

```bash
# コンテナ内のシェルに接続
docker-compose exec saba-dev bash

# 各章のSaBaブラウザを実行
./docker/run_docker.sh ch6/saba  # 第6章のブラウザ
./docker/run_docker.sh ch7/saba  # 第7章のブラウザ
```

### 利用可能な章

- `ch0/saba` - 準備章
- `ch2/saba` - 第2章（URL解析）
- `ch3/saba` - 第3章（HTTP実装）
- `ch4/saba` - 第4章（HTML解析）
- `ch5/saba` - 第5章（CSS装飾）
- `ch6/saba` - 第6章（GUI実装）
- `ch7/saba` - 第7章（JavaScript実行）

## 開発の流れ

1. **ホスト側でコード編集**: 好きなエディタ/IDEでソースコードを編集
2. **コンテナ内でビルド・実行**: Docker環境でビルドと実行
3. **ブラウザでGUI確認**: http://localhost:6080 でQEMU画面を確認

## よくある問題と解決策

### Q: ブラウザで http://localhost:6080 にアクセスできない

**A**: 以下を確認してください：
- コンテナが正常に起動しているか: `docker-compose ps`
- ポートが正しく公開されているか: `docker-compose logs saba-dev`
- ファイアウォールがポート6080をブロックしていないか

### Q: VNC画面が表示されない

**A**: 以下を試してください：
```bash
# コンテナを再起動
docker-compose restart

# ログを確認
docker-compose logs saba-dev
```

### Q: QEMUが起動しない

**A**: 以下を確認してください：
- コンテナが特権モードで実行されているか（docker-compose.ymlで設定済み）
- 十分なメモリが割り当てられているか

### Q: ビルドが失敗する

**A**: 以下を試してください：
```bash
# キャッシュをクリアして再ビルド
docker-compose build --no-cache

# 古いイメージとコンテナを削除
docker system prune -a
```

### Q: 動作が重い

**A**: Docker Desktopの設定を調整してください：
- **メモリ**: 6GB以上に設定
- **CPU**: 4コア以上に設定
- **ディスク**: 十分な容量を確保

## コンテナの管理

### コンテナの停止

```bash
docker-compose down
```

### コンテナの再起動

```bash
docker-compose restart
```

### ログの確認

```bash
docker-compose logs saba-dev
```

### コンテナ内のシェルに接続

```bash
docker-compose exec saba-dev bash
```

## トラブルシューティング

### 完全なリセット

問題が解決しない場合は、以下で完全にリセットできます：

```bash
# コンテナとボリュームを削除
docker-compose down -v

# イメージを削除
docker rmi sababook-saba-dev

# 再ビルド
docker-compose build
docker-compose up -d
```

### パフォーマンス最適化

Docker Desktopの設定で以下を調整してください：

- **Resources > Advanced**:
  - CPUs: 4以上
  - Memory: 6GB以上
  - Swap: 2GB以上

## 技術仕様

- **ベースイメージ**: Ubuntu 22.04
- **Rustバージョン**: nightly-2024-01-01
- **QEMUバージョン**: 6.2以降
- **VNCポート**: 5900
- **noVNCポート**: 6080
- **画面解像度**: 1280x800

## サポート

問題が発生した場合は、以下の情報を含めてIssueを作成してください：

1. 使用しているOS
2. Docker Desktopのバージョン
3. エラーメッセージ
4. `docker-compose logs saba-dev` の出力

## 次のステップ

Docker環境が正常に動作したら、本書の各章を順番に進めてください。各章のコードはホスト側で編集し、Docker環境でビルド・実行することで、統一された環境でブラウザ開発を学習できます。
