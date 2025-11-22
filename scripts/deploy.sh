#!/bin/bash
set -e  # エラーで即座に終了

# 環境変数の確認
: "${DEPLOY_SERVER:?DEPLOY_SERVER environment variable is required}"
: "${DEPLOY_USER:?DEPLOY_USER environment variable is required}"
# DEPLOY_PATHはデフォルト値を設定
DEPLOY_PATH="${DEPLOY_PATH:-Projects}"

# カラー出力
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Starting deployment to ${DEPLOY_SERVER}${NC}"
echo -e "${GREEN}========================================${NC}"

# デプロイ先の設定
REMOTE_HOST="${DEPLOY_SERVER}"
REMOTE_USER="${DEPLOY_USER}"
REMOTE_BASE_DIR="/home/${REMOTE_USER}/${DEPLOY_PATH}"
REMOTE_APP_DIR="${REMOTE_BASE_DIR}/umaibou-monster-game-server"
BACKUP_DIR="${REMOTE_BASE_DIR}/umaibou-monster-game-server.backup"

# SSHコマンドのラッパー（オプションを一元管理）
# StrictHostKeyChecking=no はTeleportのProxy経由の場合に便利だが、
# teleport-actions/auth が known_hosts を管理してくれる場合は不要なこともある。
# ここでは念のため安全側に倒してデフォルトの挙動に任せるが、
# 接続エラーが出る場合は -o StrictHostKeyChecking=no を検討。
SSH_CMD="ssh"
SCP_CMD="scp"

# 1. リモートサーバーでディレクトリ準備
echo -e "${YELLOW}Step 1: Preparing remote directory...${NC}"
$SSH_CMD ${REMOTE_USER}@${REMOTE_HOST} << EOF
    mkdir -p ${REMOTE_APP_DIR}

    # 既存のバックアップがあれば削除
    if [ -d "${BACKUP_DIR}" ]; then
        rm -rf "${BACKUP_DIR}"
    fi
EOF

# 2. 実行中のサービスを停止（存在する場合）
echo -e "${YELLOW}Step 2: Stopping existing service...${NC}"
$SSH_CMD ${REMOTE_USER}@${REMOTE_HOST} << EOF
    if pgrep -f umaibou-monster-game-server > /dev/null; then
        echo "Stopping umaibou-monster-game-server..."
        pkill -TERM -f umaibou-monster-game-server || true
        sleep 2
        if pgrep -f umaibou-monster-game-server > /dev/null; then
            pkill -KILL -f umaibou-monster-game-server || true
        fi
        echo "Service stopped"
    else
        echo "Service is not running"
    fi
EOF

# 3. 現在のバージョンをバックアップ
echo -e "${YELLOW}Step 3: Creating backup of current version...${NC}"
$SSH_CMD ${REMOTE_USER}@${REMOTE_HOST} << EOF
    if [ -d "${REMOTE_APP_DIR}/target" ] || [ -f "${REMOTE_APP_DIR}/umaibou-monster-game-server" ]; then
        echo "Creating backup..."
        cp -r ${REMOTE_APP_DIR} ${BACKUP_DIR}
        echo "Backup created"
    else
        echo "No existing installation found, skipping backup"
    fi
EOF

# 4. バイナリとアセットをアップロード
echo -e "${YELLOW}Step 4: Uploading new version...${NC}"

# バイナリをアップロード
echo "Uploading binary..."
$SCP_CMD target/release/umaibou-monster-game-server ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_APP_DIR}/

# マイグレーションファイルをアップロード
if [ -d "migrations" ]; then
    echo "Uploading migrations..."
    $SCP_CMD -r migrations ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_APP_DIR}/
fi

# dataディレクトリをアップロード
if [ -d "data" ]; then
    echo "Uploading data directory..."
    $SCP_CMD -r data ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_APP_DIR}/
fi

# 5. パーミッション設定
echo -e "${YELLOW}Step 5: Setting permissions...${NC}"
$SSH_CMD ${REMOTE_USER}@${REMOTE_HOST} << EOF
    cd ${REMOTE_APP_DIR}
    chmod +x umaibou-monster-game-server
    echo "Permissions set"
EOF

# 6. データベースマイグレーション実行
echo -e "${YELLOW}Step 6: Running database migrations...${NC}"
$SSH_CMD ${REMOTE_USER}@${REMOTE_HOST} << 'EOF'
    cd ~/Projects/umaibou-monster-game-server
    if [ -f "umaibou-monster-game-server" ] && [ -d "migrations" ]; then
        export DATABASE_URL="${DATABASE_URL:-sqlite://data/game.db}"
        echo "Migration files deployed. Run 'sqlx migrate run' manually if needed."
    fi
EOF

# 7. サービスを起動
echo -e "${YELLOW}Step 7: Starting service...${NC}"
$SSH_CMD ${REMOTE_USER}@${REMOTE_HOST} << EOF
    cd ${REMOTE_APP_DIR}
    nohup ./umaibou-monster-game-server > server.log 2>&1 &
    echo \$! > server.pid
    echo "Service started with PID: \$(cat server.pid)"
EOF

# 8. ヘルスチェック
echo -e "${YELLOW}Step 8: Health check...${NC}"
sleep 3

$SSH_CMD ${REMOTE_USER}@${REMOTE_HOST} << EOF
    cd ${REMOTE_APP_DIR}
    if pgrep -f umaibou-monster-game-server > /dev/null; then
        echo "✓ Service is running"
        if ss -tuln | grep -q :8080; then
            echo "✓ Service is listening on port 8080"
        else
            echo "⚠ Warning: Port 8080 is not listening yet"
        fi
    else
        echo "✗ Service failed to start"
        tail -n 20 server.log
        exit 1
    fi
EOF

echo -e "${GREEN}Deployment completed successfully!${NC}"
