use std::{time::{Duration, SystemTime}, thread::sleep};
use crate::settings::types::RandomGeneratorSettings;
use super::types::Generate;
use log::error;
use rand::{distributions::{Bernoulli, Alphanumeric}, Rng};

pub struct RandomGenerator {
    pub settings: RandomGeneratorSettings,
}

impl RandomGenerator {
    pub fn new(settings: RandomGeneratorSettings) -> Self {
        RandomGenerator { 
            settings: settings
        }
    }
}

impl super::types::Generate for RandomGenerator {
    fn generate(&self) -> Vec<u8> {
        super::GENERATED_MESSAGES_COUNT.inc();
        Vec::from(rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(self.settings.message_size_in_bytes as usize)
            .map(char::from)
            .collect::<String>()
            .as_bytes())
    }
}

impl super::types::GeneratorLoop for RandomGenerator {
    fn run_generation_loop(
        &self,
        sender: crossbeam_channel::Sender<Vec<u8>>
    ) {
        let global_start_time = SystemTime::now();
        let max_duration = Duration::from_secs(self.settings.duration_in_sec);
        let dist = Bernoulli::new(self.settings.burst_probability).unwrap();
        let mut rng = rand::thread_rng();
        let base_wait_time = Duration::from_nanos((1e9 / self.settings.messages_per_sec as f64) as u64);
        let burst_wait_time = Duration::from_nanos((1e9 / self.settings.burst_messages_per_sec as f64) as u64);
        let mut generation_avg_time: u128 = 0;
        let mut is_bursting = false;
        let mut start_burst_time = SystemTime::now();

        while global_start_time.elapsed().unwrap() < max_duration {
            let start_time = SystemTime::now();
            let msg = self.generate();
            if sender.send(msg).is_err() {
                error!("Could not send message");
            }
            if rng.sample(dist) {
                is_bursting = true;
                start_burst_time = SystemTime::now();
            };
            if is_bursting & (start_burst_time.elapsed().unwrap() <= Duration::from_millis(1000)){
                //let to_sub = Duration::from_nanos(generation_avg_time as u64);
                sleep(burst_wait_time);
            } else {
                is_bursting = false;
                sleep(base_wait_time);
            }
            generation_avg_time = (generation_avg_time + start_time.elapsed().unwrap().as_nanos()) / super::GENERATED_MESSAGES_COUNT.get() as u128;
        }
        // Sending an empty message is the signal to stop producing
        sender.send(vec![]).unwrap();
    }
}

