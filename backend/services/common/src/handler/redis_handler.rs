use crate::models::config::app_config::get_config;
use redis::Commands;
use std::error::Error as StdError;

use super::error_display::ParseError;

#[derive(Clone)]
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
        .map_err(|_| Box::new(ParseError::from("Redis client error:")) as Box<dyn StdError>)?;

        Ok(RedisHandler { client })
    }

    pub fn insert_update_key(
        &self,
        key: &String,
        value: &String,
    ) -> Result<bool, Box<dyn StdError>> {
        let mut con = self.client.get_connection()?;
        let _: () = con.set(key, value)?;
        Ok(true)
    }

    pub fn get_key(&self, key: &str) -> Result<String, Box<dyn StdError>> {
        let mut con = self.client.get_connection()?;
        let result: Option<String> = con.get(key)?;
        match result {
            Some(value) => Ok(value),
            None => Err("Value not found".into()),
        }
    }

    pub fn remove_key(&self, key: &str) -> Result<bool, Box<dyn StdError>> {
        let mut con = self.client.get_connection()?;
        let result: i32 = con.del(key)?;
        Ok(result > 0)
    }
}
