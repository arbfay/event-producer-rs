use async_trait::async_trait;
use log::info;
use redis::{RedisError, AsyncCommands};
use crate::settings::types::RedisSettings;
use super::types::{Produce, ImplOutput};

pub struct RedisProducer{
    _settings: RedisSettings,
    client: Option<redis::Client>,
    conn: Option<redis::aio::Connection>
}

impl Default for RedisProducer {
    fn default() -> Self {
        RedisProducer{
            _settings: RedisSettings { uri: "".to_string(), n_conn: 0, mode: crate::settings::types::RedisMode::PubSub },
            client: None,
            conn: None
        }
    }
}

impl RedisProducer {
    pub fn instantiate<'a>(&mut self, settings: RedisSettings) {
        info!("Creating a Redis Producer");
        let client = redis::Client::open(settings.uri.clone()).expect("Failed to start Redis client");
        //let conn = client.get_async_connection().await.unwrap();
        self._settings = settings;
        self.client = Some(client);
        //self.conn = Some(None);

    }
}

#[async_trait]
impl Produce for RedisProducer {
    type Output = ImplOutput;
    async fn produce(&mut self, message: Vec<u8>) -> Self::Output {
        // Pubsub and synchronous
        if self.conn.is_none(){
            self.conn = Some(self.client.as_ref().unwrap().get_async_connection().await.unwrap());
        }
        //let mut conn = self.conn.expect("Failed to connect to Redis");
        let res: Result<(), RedisError> = self.conn.as_mut().unwrap().publish("channel_1", message).await;
        match res {
            Ok(()) => std::future::ready(Ok(super::PRODUCTION_MESSAGES_SENT.inc())),
            Err(err) => {
                super::PRODUCTION_MESSAGES_FAILED_TO_SEND.inc();
                std::future::ready(Err(format!("{}", err)))
            }
        }
    }
}

