use c3k_common::handler::error_display::ParseError;
use c3k_common::models::auth::{AuthModel, JwtClaims, SignupUsers, UserProducts};
use c3k_common::utilities::security_utils::SecurityUtils;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde_json;
use sqlx::PgPool;
use std::error::Error as StdError;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::models::role::user_role_maps::UserRoleMaps;
use crate::repositories::role::user_role_maps::UserRoleMapsRepository;
use crate::{
    models::role::users::Users,
    repositories::role::{products::ProductsRepository, users::UsersRepository},
};
use c3k_common::{
    handler::redis_handler::RedisHandler,
    interfaces::irepository::IRepository,
    models::{config::app_config::get_config, response::ApiResponse},
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

    fn generate_jwt(
        &self,
        user: &Users,
        claims: &Vec<UserProducts>,
    ) -> Result<String, Box<dyn StdError>> {
        let json = match get_config() {
            Some(cfg) => cfg,
            None => return Err(Box::new(ParseError::from("Configuration error"))),
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
            role: claims
                .iter()
                .map(|p| format!("{}-{}", user.username, p.abbreviation))
                .collect::<Vec<String>>(),
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

        let config = match get_config() {
            Some(cfg) => cfg,
            None => return Err("Internal error: Configuration not initialized".into()),
        };

        if &SecurityUtils::generate_hash(
            password,
            &user.salt,
            &config.token_provider.token_security_algorithm,
        )? != &user.password
        {
            return Err("Invalid password".into());
        }

        let products = match ProductsRepository::get_products(self.db_pool.clone(), username).await
        {
            Ok(products) => products,
            Err(e) => return Err(e.into()),
        };

        match self.generate_jwt(user, &products) {
            Ok(token) => Ok({
                self.redis_client
                    .insert_update_key(&user.username, &token)?;

                for product in products {
                    let claims = match ProductsRepository::get_claims(
                        self.db_pool.clone(),
                        username,
                        &product.full_name,
                    )
                    .await
                    {
                        Ok(claims) => claims,
                        Err(e) => return Err(e.into()),
                    };

                    self.redis_client.insert_update_key(
                        &format!("{}-{}", user.username, product.abbreviation),
                        &serde_json::to_string(&claims).unwrap(),
                    )?;
                }

                token
            }),
            Err(e) => Err(e.into()),
        }
    }

    async fn remove_chache(&self, username: &String) -> Result<bool, Box<dyn StdError>> {
        // Fetch products associated with the username
        let products = match ProductsRepository::get_products(self.db_pool.clone(), username).await
        {
            Ok(products) => products,
            Err(e) => return Err(e.into()), // Return the error if fetching fails
        };

        // Iterate through each product and remove its cache
        for product in products {
            if !self
                .redis_client
                .remove_key(&format!("{}-{}", username, product.abbreviation))?
            {
                return Ok(false); // Return false if any key removal fails
            }
        }

        // Remove the main cache key associated with the username
        if !self.redis_client.remove_key(username)? {
            return Ok(false); // Return false if the main key removal fails
        }

        // If all operations succeed, return true
        Ok(true)
    }

    async fn create_user_with_roles(
        &self,
        entity: &SignupUsers,
    ) -> Result<String, Box<dyn StdError>> {
        let user = Users {
            user_id: entity.user_id,
            username: entity.username.clone(),
            display_name: entity.display_name.clone(),
            language: entity.language.clone(),
            password: entity.password.clone(),
            salt: "".to_string(),
            status_id: entity.status_id,
        };

        // Try inserting the user into the database
        if let Err(e) = UsersRepository::add(self.db_pool.clone(), &user).await {
            return Err(e);
        }

        // Insert roles for the user
        for role in &entity.roles {
            let temp_role = UserRoleMaps {
                user_role_map_id: Uuid::new_v4(),
                role_id: *role,
                user_id: entity.user_id,
                status_id: entity.status_id,
            };

            if let Err(e) = UserRoleMapsRepository::add(self.db_pool.clone(), &temp_role).await {
                return Err(e);
            }
        }

        Ok("User has been created Successfully".to_string())
    }

    pub async fn signup(&self, entity: &SignupUsers) -> ApiResponse<String> {
        match self.create_user_with_roles(entity).await {
            Ok(response) => ApiResponse::success(response),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    pub async fn login(&self, entity: &AuthModel) -> ApiResponse<String> {
        match self.validate(&entity.username, &entity.password).await {
            Ok(response) => ApiResponse::success(response),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    pub async fn logout(&self, entity: &AuthModel) -> ApiResponse<bool> {
        match self.remove_chache(&entity.username).await {
            Ok(response) => ApiResponse::success(response),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
