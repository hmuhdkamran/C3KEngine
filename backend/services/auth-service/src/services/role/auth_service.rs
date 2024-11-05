use c3k_common::models::auth::{Auth, AuthModel, JwtClaims, PasswordCode};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::error::Error as StdError;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    models::role::users::Users,
    repositories::role::{auth_repository::AuthRepository, users::UsersRepository},
};
use c3k_common::{
    handler::redis_handler::RedisHandler,
    interfaces::irepository::IRepository,
    models::{config::app_config::get_json, response::ApiResponse},
};

pub struct AuthService {
    pub redis_client: RedisHandler,
    pub db_pool: PgPool,
}

impl AuthService {
    pub fn new(db_pool: PgPool, redis_client: RedisHandler) -> Self {
        Self {
            db_pool,
            redis_client,
        }
    }

    fn generate_hash(data: &String, salt: &String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hasher.update(salt.as_bytes());
        let hash_result = hasher.finalize();
        format!("{:x}", hash_result)
    }

    fn generate_salt(length: usize) -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }

    fn generate_jwt(&self, user: &Users, claims: &Vec<Auth>) -> Result<String, Box<dyn StdError>> {
        let json = match get_json() {
            Ok(cfg) => cfg,
            Err(err) => return Err(err),
        };

        let now = SystemTime::now();
        let unix_time = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let claims = JwtClaims {
            aud: json.token_provider.token_audience.to_string(),
            expiry: unix_time + json.token_provider.token_expiration,
            sid: user.username.to_string(),
            emailaddress: user.username.to_string(),
            name: vec![user.username.to_string(), user.display_name.to_string()],
            role: claims.clone(),
            culturename: "en-US".to_string(),
            iss: json.token_provider.token_issuer.to_string(),
            sub: user.username.to_string(),
            typ: "JWT".to_string(),
            exp: unix_time + json.token_provider.token_expiration,
            iat: unix_time,
        };

        let header = Header::new(Algorithm::HS256);
        let secret = EncodingKey::from_secret(json.token_provider.token_security_key.as_bytes());
        let token = encode(&header, &claims, &secret)?;

        self.redis_client
            .insert_update_key(&user.username, &token)?;

        Ok(token)
    }

    async fn validate(
        &self,
        username: &String,
        password: &String,
    ) -> Result<String, Box<dyn StdError>> {
        let user_query = format!(r#""Username"='{}'"#, username);
        let entities = match UsersRepository::get_by_filter(self.db_pool.clone(), &user_query).await
        {
            Ok(entities) => entities,
            Err(e) => return Err(e.into()),
        };

        if entities.is_empty() {
            return Err("User not found".into());
        }

        let user = &entities[0];

        if &Self::generate_hash(password, &user.salt) != &user.password {
            return Err("Invalid password".into());
        }

        let claims = match AuthRepository::get_claims(self.db_pool.clone(), username).await {
            Ok(claims) => claims,
            Err(e) => return Err(e.into()),
        };

        match self.generate_jwt(user, &claims) {
            Ok(token) => Ok(token),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn login(&self, entity: &AuthModel) -> ApiResponse<String> {
        match self.validate(&entity.username, &entity.password).await {
            Ok(response) => ApiResponse::success(response),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    pub fn encrypt_password(password: &String) -> PasswordCode {
        let salt = Self::generate_salt(16);
        let hash = Self::generate_hash(password, &salt);
        PasswordCode {
            password: hash,
            salt: salt,
        }
    }
}
