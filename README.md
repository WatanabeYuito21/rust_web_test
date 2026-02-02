# Rust ダッシュボード

Axumフレームワークを使用したRust製のウェブダッシュボードアプリケーションです。

## 概要

このプロジェクトは、Rustの高速性と安全性を活かしたウェブダッシュボードアプリケーションです。複数のユーザーアカウントに対応した認証システム、システム情報のリアルタイム表示、暗号化/復号化機能など、実用的な機能を備えています。

## 主な機能

### 認証・ユーザー管理
- **セキュアな認証システム**: Argon2によるパスワードハッシュ化でセキュアなログイン/ログアウト機能を提供
- **複数アカウント対応**: MySQLデータベースを使用した複数ユーザーの管理
- **ユーザー一覧表示**: 登録されているユーザーの一覧を表示
- **セッション管理**: tower-sessionsによる安全なセッション管理

### セキュリティ・監査機能
- **ロールベースアクセス制御（RBAC）**:
  - **Admin**: すべての機能にアクセス可能（ユーザー管理、監査ログ閲覧、システム情報など）
  - **User**: 一般機能にアクセス可能（暗号化/復号化、システム情報閲覧など）
  - **Viewer**: 読み取り専用（ホーム、アバウト、時刻表示のみ）
  - 権限に応じた自動的なアクセス制御とログ記録

- **監査ログ/操作ログ**:
  - ユーザーの操作履歴を自動記録
  - ログイン/ログアウト、暗号化/復号化などの重要操作を追跡
  - 権限エラー（access_denied）も記録してセキュリティ監視を強化
  - 最新100件のログを表示

### ツール機能
- **暗号化/復号化ツール**:
  - AES-256-GCMアルゴリズムを使用した強力な暗号化
  - パスワードベースの暗号化で簡単に利用可能
  - Argon2によるキー導出で安全性を確保

### システム情報
- **システム情報表示**: CPU、メモリ、ディスク使用率などのシステム情報を表示
- **リアルタイム更新**: Server-Sent Events (SSE) を使用したリアルタイム更新機能
- **現在時刻表示**: サーバーの現在時刻を表示

## 技術スタック

### バックエンド
- **Rust** - 高速で安全なシステムプログラミング言語（2024エディション）
- **Axum** - モダンで使いやすいウェブフレームワーク
- **Tokio** - 高性能な非同期ランタイム

### データベース
- **SQLx** - コンパイル時に型チェックされるMySQLデータベース接続
- **MySQL** - リレーショナルデータベース

### テンプレート・静的ファイル
- **Askama** - Rust用の型安全なテンプレートエンジン
- **tower-http** - 静的ファイル配信

### セキュリティ・暗号化
- **Argon2** - パスワードハッシュ化とキー導出
- **AES-GCM** - AES-256-GCMによる認証付き暗号化

### セッション管理
- **tower-sessions** - セキュアなセッション管理（メモリストア使用）

### その他
- **sysinfo** - クロスプラットフォームのシステム情報取得

## 必要な環境

- **Rust** 2024エディション以降
- **MySQL** サーバー（5.7以降推奨）
- **Cargo** パッケージマネージャー

## インストールと実行手順

### 1. リポジトリのクローン

```bash
git clone <repository-url>
cd rust_web_test
```

### 2. データベースのセットアップ

MySQLにログインしてデータベースを作成します。

```bash
# MySQLにログイン
mysql -u root -p

# 以下のコマンドをMySQLプロンプトで実行
```

```sql
-- データベースを作成
CREATE DATABASE rust_dashboard;
USE rust_dashboard;

-- マイグレーションSQLを実行
SOURCE migrations/001_create_users_table.sql;
SOURCE migrations/002_create_audit_logs_table.sql;
SOURCE migrations/003_add_role_to_users.sql;
```

### 3. 環境変数の設定

プロジェクトルートに`.env`ファイルを作成し、データベース接続情報を設定します。

```bash
# .env.exampleをコピー（存在する場合）
cp .env.example .env
```

`.env`ファイルを以下のように編集します。

```env
DATABASE_URL=mysql://ユーザー名:パスワード@localhost:3306/rust_dashboard
```

例：
```env
DATABASE_URL=mysql://root:password@localhost:3306/rust_dashboard
```

### 4. アプリケーションのビルドと実行

```bash
# 依存関係のインストールとビルド
cargo build

# アプリケーションの起動
cargo run
```

サーバーが起動すると、`http://localhost:3000` でアクセスできます。

### 5. ユーザーの追加

初回起動時はユーザーが登録されていないため、以下のコマンドで新しいユーザーを追加します。

```bash
cargo run --bin add_user
```

プロンプトに従ってユーザー名とパスワードを入力すると、データベースに登録されます。

#### パスワードハッシュの生成（オプション）

パスワードのハッシュ値だけを生成したい場合は、以下のコマンドを使用します。

```bash
cargo run --bin hash
```

生成されたハッシュ値を直接データベースに挿入することもできます。

## プロジェクト構成

```
.
├── Cargo.toml                      # プロジェクト設定と依存関係
├── .env                            # 環境変数設定（自分で作成）
├── migrations/                     # データベースマイグレーション
│   ├── 001_create_users_table.sql # ユーザーテーブル作成SQL
│   ├── 002_create_audit_logs_table.sql # 監査ログテーブル作成SQL
│   └── 003_add_role_to_users.sql  # ユーザーロール追加SQL
├── src/
│   ├── main.rs                     # アプリケーションのエントリーポイント
│   ├── lib.rs                      # ライブラリのエントリーポイント
│   ├── db.rs                       # データベース操作関数
│   ├── bin/                        # バイナリユーティリティ
│   │   ├── hash.rs                 # パスワードハッシュ生成ツール
│   │   └── add_user.rs             # ユーザー追加ツール
│   └── routes/                     # ルートハンドラ
│       ├── mod.rs                  # ルートモジュール定義
│       ├── auth.rs                 # 認証機能（ログイン/ログアウト）
│       ├── audit.rs                # 監査ログ表示
│       ├── home.rs                 # ホームページとアバウトページ
│       ├── sysinfo.rs              # システム情報表示
│       ├── time.rs                 # 現在時刻表示
│       ├── users.rs                # ユーザー一覧表示
│       └── crypto.rs               # 暗号化/復号化ツール
├── static/                         # 静的ファイル
│   └── style.css                   # スタイルシート
└── templates/                      # Askamaテンプレート
    ├── base.html                   # ベーステンプレート
    ├── index.html                  # ホームページ
    ├── about.html                  # アバウトページ
    ├── login.html                  # ログインページ
    ├── audit.html                  # 監査ログページ
    ├── sysinfo.html                # システム情報ページ
    ├── users.html                  # ユーザー一覧ページ
    ├── crypto.html                 # 暗号化/復号化ページ
    └── partials/                   # パーシャルテンプレート
        └── time.html               # 時刻表示パーシャル
```

## APIエンドポイント一覧

| パス | メソッド | 説明 | 認証 | 必要な権限 |
|------|----------|------|------|-----------|
| `/` | GET | ホームページ | 必要 | すべて |
| `/about` | GET | アバウトページ | 必要 | すべて |
| `/time` | GET | サーバーの現在時刻を表示 | 必要 | すべて |
| `/sysinfo` | GET | システム情報を表示 | 必要 | User以上 |
| `/sysinfo/live` | GET | システム情報のリアルタイム更新（SSE） | 必要 | User以上 |
| `/users` | GET | 登録されているユーザーの一覧を表示 | 必要 | Admin |
| `/audit` | GET | 監査ログを表示（最新100件） | 必要 | Admin |
| `/crypto` | GET | 暗号化/復号化ツールページ | 必要 | User以上 |
| `/crypto/encrypt` | POST | テキストを暗号化 | 必要 | User以上 |
| `/crypto/decrypt` | POST | テキストを復号化 | 必要 | User以上 |
| `/login` | GET | ログインページを表示 | 不要 | なし |
| `/login` | POST | ログイン処理を実行 | 不要 | なし |
| `/logout` | GET | ログアウト処理を実行 | 必要 | すべて |

### 暗号化/復号化APIの使用方法

#### 暗号化

```bash
curl -X POST http://localhost:3000/crypto/encrypt \
  -d "plaintext=秘密のメッセージ" \
  -d "password=your-password"
```

#### 復号化

```bash
curl -X POST http://localhost:3000/crypto/decrypt \
  -d "ciphertext=<暗号化されたテキスト>" \
  -d "password=your-password"
```

## セキュリティについて

このアプリケーションは以下のセキュリティ対策を実装しています。

- **パスワードハッシュ化**: Argon2アルゴリズムを使用したセキュアなパスワードハッシュ化
- **セッション管理**: tower-sessionsによる安全なセッション管理
- **認証保護**: ログインページ以外のすべてのページで認証が必要
- **ロールベースアクセス制御**: Admin/User/Viewerの3段階のロールによる細やかな権限管理
- **暗号化**: AES-256-GCMによる強力な暗号化とArgon2によるキー導出
- **監査ログ**: ユーザーの操作履歴と権限エラーを記録し、セキュリティ監視とコンプライアンス対応を支援

### ユーザーロールについて

新規ユーザーはデフォルトで「User」ロールで作成されます。管理者権限が必要な場合は、データベースで直接ロールを変更してください。

```sql
-- ユーザーを Admin に変更
UPDATE users SET role = 'admin' WHERE username = 'your-username';

-- ユーザーを Viewer に変更
UPDATE users SET role = 'viewer' WHERE username = 'your-username';
```

既存の「admin」ユーザーは自動的に Admin ロールに設定されます。

## 開発

### テスト実行

```bash
cargo test
```

### 開発モードでの実行

```bash
# デバッグビルドで実行
cargo run

# リリースビルドで実行
cargo run --release
```

## ライセンス

MIT
