use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub  fn hash_password(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password, &salt)
        .expect("Unable to hash password.")
        .to_string()
}

pub fn verify_password(
    hash: &str,
    password: &[u8],
) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Argon2::default().verify_password(password, &parsed_hash)
}

// #[test]
// fn verification_succeeded() {
//     let hash = hash_password(String::from("123")).unwrap();
//     let hash_verification = verify_password(hash, String::from("123")).unwrap();
//     assert_eq!(hash_verification, true);
// }

// #[test]
// fn verification_failed() {
//     let hash = hash_password(String::from("123")).unwrap();
//     let bad_hash_verification = verify_password(hash, String::from("xnpgu")).unwrap();
//     assert_eq!(bad_hash_verification, false);
// }