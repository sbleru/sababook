# SaBaブラウザ開発環境
FROM ubuntu:22.04

# 非対話的インストールのための環境変数
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Asia/Tokyo

# 基本パッケージのインストール
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    wget \
    make \
    qemu-system-x86 \
    pkg-config \
    libssl-dev \
    ca-certificates \
    tzdata \
    # VNCとGUI関連パッケージ
    tigervnc-standalone-server \
    tigervnc-common \
    fluxbox \
    xterm \
    python3 \
    python3-pip \
    python3-numpy \
    net-tools \
    && rm -rf /var/lib/apt/lists/*

# noVNCのインストール
RUN git clone https://github.com/novnc/noVNC.git /opt/noVNC \
    && git clone https://github.com/novnc/websockify /opt/noVNC/utils/websockify \
    && ln -s /opt/noVNC/vnc.html /opt/noVNC/index.html

# Rustのインストール（nightly-2024-01-01）
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly-2024-01-01
ENV PATH="/root/.cargo/bin:${PATH}"

# Rustのターゲット追加
RUN rustup target add x86_64-unknown-none

# VNCの設定
RUN mkdir -p /root/.vnc \
    && echo "saba123" | vncpasswd -f > /root/.vnc/passwd \
    && chmod 600 /root/.vnc/passwd


# entrypointスクリプトをコピー
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# 作業ディレクトリの設定
WORKDIR /workspace

# ポートの公開
EXPOSE 5900 6080

# entrypointの設定
ENTRYPOINT ["/entrypoint.sh"]
CMD ["/bin/bash"]

