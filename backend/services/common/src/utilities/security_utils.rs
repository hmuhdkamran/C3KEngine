use jsonwebtoken::{errors::{ ErrorKind, Error as JError }, Algorithm, DecodingKey, Validation};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sha2::{Digest, Sha256, Sha512};
use std::error::Error;

use crate::models::auth::JwtClaims;

pub struct SecurityUtils;

impl SecurityUtils {
    /// Generates a SHA256 hash for the given data and salt.
    pub fn generate_hash(
        data: &str,
        salt: &str,
        algorithm: &str,
    ) -> Result<String, Box<dyn Error>> {
        let salted_data = format!("{}{}", data, salt);

        match algorithm.to_uppercase().as_str() {
            "SHA256" => {
                let mut hasher = Sha256::new();
                hasher.update(salted_data.as_bytes());
                Ok(format!("{:x}", hasher.finalize()))
            }
            "SHA512" => {
                let mut hasher = Sha512::new();
                hasher.update(salted_data.as_bytes());
                Ok(format!("{:x}", hasher.finalize()))
            }
            _ => Err(format!("Unsupported hashing algorithm: {}", algorithm).into()),
        }
    }

    /// Generates a random alphanumeric salt of the given length.
    pub fn generate_salt(length: usize) -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }

    /// Encrypts a password by generating a salt and hashing the password with it.
    pub fn encrypt_password(
        password: &str,
        length: usize,
        algorithm: &str,
    ) -> Result<(String, String), Box<dyn Error>> {
        let salt = Self::generate_salt(length);
        let hash = Self::generate_hash(password, &salt, algorithm)?;
        Ok((hash, salt))
    }

    /// Verifies a JWT token and validates its claims.
    pub fn verify_token(
        token: &str,
        secret: &str,
        audience: &str,
        algorithm: &str,
    ) -> Result<JwtClaims, JError> {
        let alg = match algorithm.to_uppercase().as_str() {
            "SHA256" => Algorithm::HS256,
            "SHA512" => Algorithm::HS512,
            _ => {
                return Err(JError::from(ErrorKind::InvalidAlgorithm,))
            }
        };
    
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        let mut validation = Validation::new(alg);        
        validation.set_audience(&[audience]);

        match jsonwebtoken::decode::<JwtClaims>(token, &decoding_key, &validation) {
            Ok(token_data) => {
                if token_data.claims.exp < jsonwebtoken::get_current_timestamp() {
                    return Err(ErrorKind::ExpiredSignature.into());
                }
                Ok(token_data.claims)
            }
            Err(e) => {
                eprintln!("Token Decoding Failed: {:?}", e);
                Err(e)
            }
        }
    }
    
}
