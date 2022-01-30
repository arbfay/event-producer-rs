use std::time::Duration;
use log::{error, info, debug};
use rdkafka::{producer::{Producer, BaseRecord}, util::Timeout, ClientConfig};
use crate::settings::types::KafkaSettings;
use super::types::Produce;

pub struct KafkaProducer {
    settings: KafkaSettings,
    base_producer: rdkafka::producer::BaseProducer
}

impl KafkaProducer {
    pub fn new(settings: KafkaSettings) -> Self {
        info!("Creating a Kafka Producer");
        let settings_clone = settings.clone();
        let mut base_config = ClientConfig::new();
        let mut producer_config= base_config
                    .set("bootstrap.servers", &settings.brokers.join(" "))
                    .set("api.version.request", "true")
                    .set("broker.version.fallback", "2.4.0");
        
        if settings.sasl_enabled{
            let secrets = settings.secrets.expect("No Kafka secrets found whilst SASL is enabled. It needs a username and a password.");
            producer_config = producer_config
                .set("security.protocol", "sasl_plaintext")
                .set("sasl.mechanism", "SCRAM-SHA-256")
                .set("sasl.username", secrets.username.clone().as_str())
                .set("sasl.password", secrets.password.clone().as_str());
        }

        if settings.additional_rdkafka_settings.is_some() {
            for (key, value) in settings.additional_rdkafka_settings.unwrap() {
                producer_config = producer_config.set(key, value);
            }
        }
        let producer = producer_config
                .create()
                .expect("failed to get a producer");

        KafkaProducer { 
            settings: settings_clone,
            base_producer: producer }
    }
}

impl Produce for KafkaProducer {
    fn produce(&mut self, message: Vec<u8>) -> Result<(), String>{
        super::PRODUCTION_MESSAGES_SENT.inc();
        let topic = format!("topic_{}", fastrand::u32(0..self.settings.n_topics));
        let result = match self.base_producer.send(BaseRecord::to(topic.as_str()).payload(&message).key("none")) {
            Ok(_) => Ok(()),
            Err((error, record)) => {
                let err_str = format!("Failed to enqueue. Record: {:?}. Error {}", record, error);
                error!("{}", err_str);
                super::PRODUCTION_MESSAGES_FAILED_TO_SEND.inc();
                Err(err_str)
            }
        };
        let i = self.base_producer.poll(Duration::from_millis(0));
        debug!("Producer polled {} events", i);
        result
    }
    fn stop(&self){
        self.base_producer.flush(Timeout::After(Duration::from_secs(100)));
    }
}