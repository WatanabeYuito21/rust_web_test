use argon2::Argon2;
use password_hash::{PasswordHasher, SaltString, rand_core::OsRng};
use std::io::{self, Write};

fn main() {
    print!("Password: ");
    io::stdout().flush().unwrap();

    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    println!("Hashed password: {}", hash);
}
