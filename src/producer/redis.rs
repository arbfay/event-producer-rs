use log::info;
use redis::Commands;
use crate::settings::types::RedisSettings;
use super::types::Produce;

pub struct RedisProducer{
    _settings: RedisSettings,
    conn: redis::Connection
}

impl RedisProducer {
    pub fn new(settings: RedisSettings) -> Self {
        info!("Creating a Redis Producer");
        let client = &redis::Client::open(settings.uri.clone()).expect("Failed to start Redis client");

        RedisProducer{
            _settings: settings,
            conn: client.get_connection().expect("Failed to connect to Redis"),
        }
    }
}

impl Produce for RedisProducer {
    fn produce(&mut self, message: Vec<u8>) -> Result<(), String>{
        // Pubsub and synchronous
        match self.conn.publish("channel_1", message) {
            Ok(()) => Ok(super::PRODUCTION_MESSAGES_SENT.inc()),
            Err(err) => {
                super::PRODUCTION_MESSAGES_FAILED_TO_SEND.inc();
                Err(format!("{}", err))
            }
        }
    }
}

