-- ユーザーテーブルの作成
CREATE TABLE IF NOT EXISTS users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- デフォルトユーザーの追加（パスワード: "password"）
-- このハッシュは後で適切なパスワードに変更してください
INSERT INTO users (username, password_hash) VALUES
('admin', '$argon2id$v=19$m=19456,t=2,p=1$AAS3Vkx7kZ9AowvBpcCf8g$bmJ+Ut0DWUzgQlJ7/hKs1tXcwh7c1NPPtMI6GwrZGUk');
