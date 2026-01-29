# Rust Dashboard

Axumフレームワークを使用したRust製のウェブダッシュボードアプリケーションです。

## 機能

- 認証システム（ログイン/ログアウト）
- システム情報表示
- 現在時刻表示
- セッション管理

## 技術スタック

- **Rust** - プログラミング言語
- **Axum** - Webフレームワーク
- **Tokio** - 非同期ランタイム
- **Askama** - テンプレートエンジン
- **Argon2** - パスワードハッシュ
- **tower-sessions** - セッション管理

## 必要条件

- Rust 2024 Edition

## インストール・実行

```bash
# 依存関係のインストールとビルド
cargo build

# 実行
cargo run
```

サーバーは `http://localhost:3000` で起動します。

## プロジェクト構成

```
.
├── Cargo.toml
├── src/
│   ├── main.rs          # エントリーポイント
│   ├── bin/
│   │   └── hash.rs      # パスワードハッシュユーティリティ
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

## ルート

| パス | メソッド | 説明 |
|------|----------|------|
| `/` | GET | ホームページ |
| `/about` | GET | Aboutページ |
| `/time` | GET | 現在時刻 |
| `/sysinfo` | GET | システム情報 |
| `/login` | GET/POST | ログイン |
| `/logout` | GET | ログアウト |

## ライセンス

MIT
