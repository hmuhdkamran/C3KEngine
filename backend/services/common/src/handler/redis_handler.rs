use crate::models::config::app_config::get_config;
use redis::Commands;
use std::error::Error as StdError;

use super::error_display::ParseError;

pub struct RedisHandler {
    client: redis::Client,
}

impl RedisHandler {
    pub fn new() -> Result<Self, Box<dyn StdError>> {
        let json = match get_config() {
            Some(cfg) => cfg,
            None => return Err(Box::new(ParseError::from("Configuration not loaded"))),
        };

        let client = redis::Client::open(format!(
            "redis://{}:{}/",
            json.redis.redis_host, json.redis.redis_port
        ))
        .map_err(|_| {
            Box::new(ParseError::from("Redis client error:")) as Box<dyn StdError>
        })?;

        Ok(RedisHandler { client })
    }

    pub fn insert_update_key(
        &self,
        username: &String,
        token: &String,
    ) -> Result<bool, Box<dyn StdError>> {
        let mut con = self.client.get_connection()?;
        let _: () = con.set(username, token)?;
        Ok(true)
    }

    pub fn get_key(&self, username: &str) -> Result<String, Box<dyn StdError>> {
        let mut con = self.client.get_connection()?;
        let result: Option<String> = con.get(username)?;
        match result {
            Some(value) => Ok(value),
            None => Err("Value not found".into()),
        }
    }
}
