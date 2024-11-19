use crate::models::config::app_config::get_config;
use redis::Commands;
use std::error::Error as StdError;

pub struct RedisHandler {
    client: redis::Client,
}

impl RedisHandler {
    pub fn new() -> Result<Self, Box<dyn StdError>> {
        let json = get_config().unwrap();
        let client = redis::Client::open(format!(
            "redis://{}:{}/",
            json.redis.redis_host, json.redis.redis_port
        ))?;
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
