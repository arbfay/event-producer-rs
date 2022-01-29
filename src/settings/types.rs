use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct MetricsSettings {
    pub endpoint: String,
    pub port: u16
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "sampling_distribution")]
pub enum BurstProbabilityDistribution {
    #[serde(alias="uniform")]
    Uniform,
    #[serde(alias="bernoulli")]
    Bernoulli
}

#[derive(Clone, Debug, Deserialize)]
pub struct RandomGeneratorSettings {
    pub messages_per_sec: u32,
    pub message_size_in_bytes: u64,
    pub duration_in_sec: u64,
    pub burst_messages_per_sec: u32,
    pub burst_probability: f64,
    #[serde(flatten)]
    pub sampling_distribution: BurstProbabilityDistribution
}

#[derive(Clone, Debug, Deserialize)]
pub struct GeneratorSettings {
    pub random: Option<RandomGeneratorSettings>
}

#[derive(Clone, Debug, Deserialize)]
pub struct KafkaSecrets {
    pub username: String,
    pub password: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct KafkaSettings {
    pub brokers: Vec<String>,
    pub sasl_enabled: bool,
    pub secrets: Option<KafkaSecrets>,
    pub n_topics: u32,
    pub poll_timeout_in_ms: u64,
    pub additional_rdkafka_settings: Option<HashMap<String, String>>
}

#[derive(Clone, Debug, Deserialize)]
pub struct StdoutSettings {
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProducerSettings {
    pub kafka: Option<KafkaSettings>,
    pub stdout: Option<StdoutSettings>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub metrics: MetricsSettings,
    pub generator: GeneratorSettings,
    pub producer: ProducerSettings,
}