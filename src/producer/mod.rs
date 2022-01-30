use std::time::SystemTime;
use self::types::Produce;
use lazy_static::lazy_static;
use crossbeam_channel::Receiver;
use prometheus::{IntGauge, IntCounter, Histogram, HistogramOpts, exponential_buckets};

#[cfg(feature = "kafka")]
pub mod kafka;

#[cfg(feature = "redis")]
pub mod redis;

pub mod stdout;
pub mod types;

lazy_static! {
    pub static ref PRODUCTION_CHANNEL_QUEUE: IntGauge = IntGauge::new("production_channel_queue_size", "events waiting to be processed").unwrap();
    pub static ref PRODUCTION_MESSAGES_SENT: IntCounter = IntCounter::new("production_messages_sent", "message sent to cluster").unwrap();
    pub static ref PRODUCTION_MESSAGES_FAILED_TO_SEND: IntCounter = IntCounter::new("production_messages_failed_to_send", "message failed to be sent to cluster").unwrap();
    pub static ref PRODUCTION_LATENCY_BUCKETS: Histogram = Histogram::with_opts(HistogramOpts::new("production_latency_buckets_us", "message production latency in microseconds").buckets(exponential_buckets(10.0, 2.0, 21).unwrap())).unwrap();
}

pub fn run_production_loop<'a, T: Produce>(
    producer: &'a mut T,
    receiver: Receiver<Vec<u8>>
){
    let rec_clone = receiver.clone();
    let mut counter = 0;
    let start_time = SystemTime::now();
    for msg in receiver {
        if msg.is_empty() {
            break;
        } else {
            counter+=1;
            let chrono = SystemTime::now();
            producer.produce(msg).unwrap();
            PRODUCTION_LATENCY_BUCKETS.observe(chrono.elapsed().unwrap().as_micros() as f64);
        }
        if counter%200 == 0{
            let l = rec_clone.len();
            PRODUCTION_CHANNEL_QUEUE.set(l as i64);
        }
    }
    producer.stop();
    println!("Done producing in {} seconds", start_time.elapsed().unwrap().as_secs());
}