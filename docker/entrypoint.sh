#!/bin/bash

# VNCサーバーの設定ファイルを作成
mkdir -p /root/.vnc
cat > /root/.vnc/xstartup << 'EOF'
#!/bin/bash
unset SESSION_MANAGER
unset DBUS_SESSION_BUS_ADDRESS
exec fluxbox
EOF
chmod +x /root/.vnc/xstartup

# VNCサーバーを起動
echo "Starting VNC server..."
vncserver :1 -geometry 1280x800 -depth 24 -SecurityTypes None

# noVNCを起動
echo "Starting noVNC..."
/opt/noVNC/utils/novnc_proxy --vnc localhost:5901 --listen 6080 &

# 少し待機してサービスが起動するのを待つ
sleep 3

echo "==================================="
echo "VNC Server started on port 5900"
echo "noVNC Web interface: http://localhost:6080"
echo "VNC Password: saba123"
echo "==================================="

# 引数があれば実行、なければbashを起動
if [ $# -eq 0 ]; then
    exec /bin/bash
else
    exec "$@"
fi
