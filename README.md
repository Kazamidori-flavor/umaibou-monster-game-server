# 3D Real-time Battle Game Server

actix-webで実装した3Dリアルタイム対戦ゲームのサーバーです。

## 🎯 概要

2人対戦型の3Dリアルタイムバトルゲーム用サーバー実装。
WebSocketによる60Hzのゲーム状態配信とREST APIによるマッチング機能を提供します。

## ✨ 機能

### プレイヤーマッチング

- マッチング作成・参加（WebSocket）
- マッチング一覧取得・リアルタイム更新（WebSocket）
- マッチング成功通知（WebSocket）
- キャラクター選択・準備完了（WebSocket）
- ゲーム開始通知（WebSocket）

### ゲーム進行管理

- リアルタイム操作入力受信（移動・攻撃・回転）
- ゲーム状態計算・配信（60Hz）
- 勝敗判定・ゲーム終了通知
- 戦績データ管理

## 🚀 クイックスタート

### 前提条件

- Rust 1.70以降
- Cargo

### ビルド & 起動

```bash
# リポジトリクローン（または既存プロジェクトに移動）
cd umaibou-monster-game-server

# ビルド
cargo build

# 起動
cargo run
```

サーバーは `http://0.0.0.0:8080` で起動します。

## 📡 API仕様

### REST API

#### 3Dモデル一覧取得

```bash
GET /api/models

# Response
[
  {
    "id": "model_id",
    "name": "warrior.glb",
    "is_used": false
  }
]
```

### WebSocket

#### 接続

```
ws://localhost:8080/ws?player_id={player_id}&matching_id={matching_id}
```

- `player_id`: 任意（指定なしの場合は自動生成）
- `matching_id`: 任意（再接続時に指定）

#### メッセージ型

**クライアント → サーバー:**
- `CreateMatching` - マッチング作成（`username` 指定可）
- `JoinMatch` - マッチング参加
- `Ready` - キャラクター選択・準備完了
- `Input` - 操作入力（移動・攻撃・回転）

**サーバー → クライアント:**
- `MatchingCreated` - マッチング作成完了通知
- `UpdateMatchings` - マッチング一覧更新
- `MatchingEstablished` - マッチング成立（相手決定）
- `OpponentCharacterSelected` - 相手キャラクター情報
- `GameStart` - ゲーム開始
- `OpponentStateUpdate` - 相手の状態更新
- `GameEnd` - ゲーム終了・結果
- `Error` - エラー通知

詳細は [WebSocketメッセージ仕様](doc/websocket-messages.md) を参照。

## 🧪 テスト

### 自動テスト

```bash
# ロジックテスト
cargo test --test matching_logic_test

# WebSocketテスト
cargo test --test websocket_test

# モデル使用テスト
cargo test --test model_usage_test

# 全テスト実行
cargo test
```

### 手動テスト

詳細な手順は [テスト手順書](doc/testing-guide.md) を参照。

```bash
# WebSocketクライアントインストール
npm install -g wscat

# WebSocket接続テスト
wscat -c "ws://localhost:8080/ws?player_id=player_a"
```

## 📁 プロジェクト構成

```
.
├── Cargo.toml                  # 依存関係定義
├── src/
│   ├── lib.rs                  # ライブラリエントリポイント
│   ├── main.rs                 # サーバー起動
│   ├── models.rs               # データモデル
│   ├── utils.rs                # ユーティリティ関数
│   ├── db/
│   │   ├── mod.rs
│   │   └── models.rs           # データベースモデル
│   ├── game/
│   │   ├── mod.rs
│   │   ├── state.rs            # ゲーム状態管理
│   │   └── manager.rs          # 60Hzゲームループ
│   └── handlers/
│       ├── mod.rs
│       ├── model_upload.rs     # モデルアップロードAPI
│       └── websocket.rs        # WebSocketハンドラー
├── tests/
│   ├── matching_logic_test.rs  # マッチングロジックテスト
│   ├── model_usage_test.rs     # モデル使用テスト
│   └── websocket_test.rs       # WebSocketテスト
├── scripts/
│   └── run_tests.sh            # 自動テスト実行スクリプト
└── doc/
    ├── specification.md        # 仕様書
    ├── testing-guide.md        # テスト手順書
    └── websocket-messages.md   # メッセージサンプル集
```

## 🏗️ アーキテクチャ

### 技術スタック

- **actix-web** - HTTPサーバー
- **actix-web-actors** - WebSocketサポート
- **actix** - アクターモデル（ゲームマネージャー）
- **tokio** - 非同期ランタイム
- **serde** - JSON シリアライズ
- **uuid** - ユニークID生成
- **chrono** - タイムスタンプ管理
- **sqlx** - データベース操作 (SQLite)

### 設計のポイント

#### 60Hz更新システム

```rust
// tokio::time::intervalで16.67ms間隔の高精度タイマー
ctx.run_interval(Duration::from_millis(16), |act, _ctx| {
    // ゲーム状態更新 & 配信
});
```

#### 並行処理

- **Arc<Mutex<HashMap>>** - 複数リクエスト並行処理
- **mpsc::unbounded_channel** - 非同期メッセージ配信
- **Actixアクター** - メッセージ駆動の状態管理

#### ダメージ計算

仕様に基づき、ダメージ計算はクライアント側で実施。サーバーは結果を受信して適用。

## ⚡ 非機能要件

- ✅ **60Hz状態更新** - tokio::intervalで実現
- ✅ **1000組同時処理** - 効率的な非同期処理
- ✅ **応答時間<100ms** - actix-webの高性能ランタイム
- ✅ **REST/WebSocket使い分け** - 適切なプロトコル選択

## 📝 ライセンス

このプロジェクトは個人学習用です。

## 🤝 貢献

バグ報告や機能要望は Issue でお願いします。

## 📚 参考資料

- [仕様書](doc/specification.md)
- [テスト手順書](doc/testing-guide.md)
- [WebSocketメッセージ仕様](doc/websocket-messages.md)
- [actix-web公式ドキュメント](https://actix.rs/)
