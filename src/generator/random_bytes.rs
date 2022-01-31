use std::{time::Duration, thread::sleep, iter::repeat_with};
use crate::settings::types::RandomGeneratorSettings;
use super::types::Generate;
use log::{error, info};
use dasp::Signal;

pub struct RandomGenerator {
    pub settings: RandomGeneratorSettings,
    pub rng_instance: fastrand::Rng
}

impl RandomGenerator {
    pub fn new(settings: RandomGeneratorSettings) -> Self {
        RandomGenerator { 
            settings: settings,
            rng_instance: fastrand::Rng::new()
        }
    }
}

impl super::types::Generate for RandomGenerator {
    fn generate(&self) -> Vec<u8> {
        super::GENERATED_MESSAGES_COUNT.inc();
        let s: Vec<u8> = repeat_with(|| self.rng_instance.u8(..)).take(self.settings.message_size_in_bytes as usize).collect();
        s
    }
}

impl super::types::GeneratorLoop for RandomGenerator {
    fn run_generation_loop(
        &self,
        sender: crossbeam_channel::Sender<Vec<u8>>
    ) {

        // This can take some time for long experiments, and uses most of the memory
        info!("Generating noisy signal.");
        let msg_per_sec_vec = build_noisy_signal(
        self.settings.messages_per_sec, 
        self.settings.duration_in_sec,
self.settings.burst_messages_per_sec,
self.settings.burst_probability
        );

        for n in msg_per_sec_vec {
            let base_wait_time = Duration::from_nanos( (-10.0 + (1e9 / n)) as u64);
            //let to_sub = Duration::from_nanos(generation_avg_time as u64);
            for _ in 0..(n as u64) {
                let msg = self.generate();
                let err = sender.send(msg).err();
                if err.is_some() {
                    error!("Could not send message");
                    error!("{:#?}", err.unwrap());
                }
                sleep(base_wait_time);
            }
        }
        // Sending an empty message is the signal to stop producing
        sender.send(vec![]).unwrap();
    }
}

fn build_noisy_signal(frequency: u32, length: u64, burst_frequency: u32, burst_occurence: f64) -> Vec<f64> {
    let freq = frequency as f64;
    let bfreq = burst_frequency as f64;
    dasp::signal::rate(4400.0)
                .const_hz(bfreq)
                .noise_simplex()
                .take(length as usize)
                .collect::<Vec<f64>>()
                .into_iter()
                .map(|x| {
                    let y = x + 1.0;
                    if y > (2.0 - (burst_occurence * 2.0)) {
                        y * bfreq / 2.0
                    } else {
                        y * freq
                    }
                })
                .collect()
}

