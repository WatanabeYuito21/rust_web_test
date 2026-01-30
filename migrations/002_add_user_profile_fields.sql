-- ユーザープロフィール用のフィールドを追加
ALTER TABLE users
ADD COLUMN email VARCHAR(255),
ADD COLUMN full_name VARCHAR(255);
