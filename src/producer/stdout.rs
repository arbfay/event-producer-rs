use async_trait::async_trait;
use log::info;

use crate::settings::types::StdoutSettings;
use super::types::{Produce, ImplOutput};

#[derive(Clone)]
pub struct StdoutProducer {
    _settings: StdoutSettings
}

impl Default for StdoutProducer {
    fn default() -> Self {
        StdoutProducer{
            _settings: StdoutSettings{}
        }
    }
}

impl StdoutProducer {
    pub fn instantiate(&mut self, _settings: StdoutSettings) {
        info!("Creating a Stdout Producer");
    }
}

#[async_trait]
impl Produce for StdoutProducer {
    type Output = ImplOutput;
    async fn produce(&mut self, message: Vec<u8>) -> Self::Output {
        super::PRODUCTION_MESSAGES_SENT.inc();
        println!("{:?}", message);
        std::future::ready(Ok(()))
    }
}