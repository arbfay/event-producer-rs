use crate::settings::types::StdoutSettings;
use super::types::Produce;

pub struct StdoutProducer {
    _settings: StdoutSettings
}

impl StdoutProducer {
    pub fn new(settings: StdoutSettings) -> Self {
        StdoutProducer { _settings: settings }
    }
}

impl Produce for StdoutProducer {
    fn produce(&self, message: Vec<u8>) -> Result<(), String>{
        super::PRODUCTION_MESSAGES_SENT.inc();
        println!("{:?}", message);
        Ok(())
    }
}