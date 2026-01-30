#!/bin/bash

# データベースマイグレーション実行スクリプト

echo "=== データベースマイグレーション実行 ==="
echo ""
echo "002_add_user_profile_fields.sql を実行します..."
echo ""

# .envファイルからDATABASE_URLを読み込む
if [ -f .env ]; then
    export $(cat .env | grep DATABASE_URL | xargs)
fi

if [ -z "$DATABASE_URL" ]; then
    echo "エラー: DATABASE_URL が設定されていません。"
    echo ""
    echo "MySQLに直接接続して以下のSQLを実行してください："
    echo ""
    cat migrations/002_add_user_profile_fields.sql
    echo ""
    exit 1
fi

# DATABASE_URLからMySQL接続情報を抽出
# 形式: mysql://user:password@host:port/database
DB_INFO=$(echo $DATABASE_URL | sed 's|mysql://||')
DB_USER=$(echo $DB_INFO | cut -d: -f1)
DB_PASS=$(echo $DB_INFO | cut -d: -f2 | cut -d@ -f1)
DB_HOST=$(echo $DB_INFO | cut -d@ -f2 | cut -d: -f1)
DB_PORT=$(echo $DB_INFO | cut -d: -f2 | cut -d/ -f1)
DB_NAME=$(echo $DB_INFO | cut -d/ -f2)

echo "データベース: $DB_NAME"
echo "ホスト: $DB_HOST"
echo ""

# マイグレーションを実行
mysql -h "$DB_HOST" -u "$DB_USER" -p"$DB_PASS" "$DB_NAME" < migrations/002_add_user_profile_fields.sql

if [ $? -eq 0 ]; then
    echo ""
    echo "✓ マイグレーションが正常に実行されました！"
    echo ""
    echo "以下のカラムがusersテーブルに追加されました："
    echo "  - email (VARCHAR(255))"
    echo "  - full_name (VARCHAR(255))"
else
    echo ""
    echo "✗ マイグレーションの実行に失敗しました。"
    echo ""
    echo "手動で以下のSQLを実行してください："
    echo ""
    cat migrations/002_add_user_profile_fields.sql
fi
