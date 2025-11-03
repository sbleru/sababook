# Docker環境用スクリプト

このディレクトリには、SaBaブラウザをDocker環境で実行するためのスクリプトが含まれています。

## ファイル一覧

### entrypoint.sh
- コンテナ起動時に実行されるエントリーポイントスクリプト
- VNCサーバーとnoVNCを起動
- GUI環境を初期化

### run_on_docker.sh
- Docker環境でSaBaブラウザを実行するためのスクリプト
- 各章のブラウザを簡単に起動できる


## 使用方法

### SaBaブラウザの実行

```bash
# コンテナ内で実行
./docker/run_on_docker.sh <章のディレクトリ>

# 例：第6章のブラウザを実行
./docker/run_on_docker.sh ch6/saba

# 例：第7章のブラウザを実行
./docker/run_on_docker.sh ch7/saba
```

### 利用可能な章

- `ch0/saba` - 準備章
- `ch2/saba` - 第2章（URL解析）
- `ch3/saba` - 第3章（HTTP実装）
- `ch4/saba` - 第4章（HTML解析）
- `ch5/saba` - 第5章（CSS装飾）
- `ch6/saba` - 第6章（GUI実装）
- `ch7/saba` - 第7章（JavaScript実行）

## 技術詳細

### VNC設定
- **ポート**: 5900（VNC）、6080（noVNC）
- **解像度**: 1280x800
- **パスワード**: saba123
- **ディスプレイ**: :1

### 環境変数
- `DISPLAY=:1` - VNC仮想ディスプレイを使用
- `VNC_RESOLUTION=1280x800` - 画面解像度の設定

## トラブルシューティング

### スクリプトが実行できない場合

```bash
# 実行権限を確認・付与
chmod +x docker/run_on_docker.sh
chmod +x docker/entrypoint.sh
```

### VNCが起動しない場合

```bash
# コンテナのログを確認
docker compose logs saba-dev

# コンテナを再起動
docker compose restart
```
