# Rust ダッシュボード

Axumフレームワークを使用したRust製のウェブダッシュボードアプリケーションです。

## 機能

- **複数アカウント対応の認証システム**（ログイン/ログアウト）
- MySQLデータベースによるユーザー管理
- システム情報表示
- 現在時刻表示
- セッション管理

## 技術スタック

- **Rust** - プログラミング言語
- **Axum** - ウェブフレームワーク
- **Tokio** - 非同期ランタイム
- **Askama** - テンプレートエンジン
- **SQLx** - MySQLデータベース接続
- **Argon2** - パスワードハッシュ
- **tower-sessions** - セッション管理

## 必要条件

- Rust 2024 エディション
- MySQL サーバー

## インストール・実行

### 1. データベースのセットアップ

MySQLデータベースを作成します。

```bash
# MySQLにログイン
mysql -u root -p

# データベースを作成
CREATE DATABASE rust_dashboard;
USE rust_dashboard;

# マイグレーションSQLを実行
SOURCE migrations/001_create_users_table.sql;
```

### 2. 環境変数の設定

`.env`ファイルを作成し、データベース接続情報を設定します。

```bash
cp .env.example .env
```

`.env`ファイルを編集:

```
DATABASE_URL=mysql://username:password@localhost:3306/rust_dashboard
```

### 3. アプリケーションのビルドと実行

```bash
# 依存関係のインストールとビルド
cargo build

# 実行
cargo run
```

サーバーは `http://localhost:3000` で起動します。

### 4. ユーザーの追加

新しいユーザーを追加するには、以下のコマンドを実行します。

```bash
cargo run --bin add_user
```

ユーザー名とパスワードを入力すると、データベースに登録されます。

または、パスワードハッシュツールを使用してハッシュを生成できます。

```bash
cargo run --bin hash
```

## プロジェクト構成

```
.
├── Cargo.toml
├── migrations/
│   └── 001_create_users_table.sql  # データベーススキーマ
├── src/
│   ├── main.rs          # エントリーポイント
│   ├── lib.rs           # ライブラリエントリーポイント
│   ├── db.rs            # データベース操作
│   ├── bin/
│   │   ├── hash.rs      # パスワードハッシュユーティリティ
│   │   └── add_user.rs  # ユーザー追加ツール
│   └── routes/
│       ├── mod.rs
│       ├── auth.rs      # 認証関連
│       ├── home.rs      # ホームページ
│       ├── sysinfo.rs   # システム情報
│       └── time.rs      # 時刻表示
├── static/
│   └── style.css        # スタイルシート
└── templates/           # Askamaテンプレート
    ├── base.html
    ├── index.html
    ├── about.html
    ├── login.html
    ├── sysinfo.html
    └── partials/
        └── time.html
```

## ルート一覧

| パス | メソッド | 説明 |
|------|----------|------|
| `/` | GET | ホームページ |
| `/about` | GET | アバウトページ |
| `/time` | GET | 現在時刻 |
| `/sysinfo` | GET | システム情報 |
| `/login` | GET/POST | ログイン |
| `/logout` | GET | ログアウト |

## ライセンス

MIT
