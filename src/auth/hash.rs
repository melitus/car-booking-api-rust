use argon2::{
    password_hash::{
        rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};

pub fn hash_password(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password, &salt)
        .expect("Unable to hash password.")
        .to_string()
}

pub fn verify_password(hash: &str, password: &[u8]) -> Result<bool, Error> {
    println!("Verifying password {:?} and hash {:?}", password, hash);
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default().verify_password(password, &parsed_hash).is_ok())
}
