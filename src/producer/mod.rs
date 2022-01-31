use std::{time::SystemTime, sync::Arc};
use self::types::Produce;
use lazy_static::lazy_static;
use crossbeam_channel::Receiver;
use log::info;
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

pub fn run_production_loop<'a, T>(
    producer: T,
    receiver: Receiver<Vec<u8>>
)
where
    T: 'static + Produce + Send + Sync,
    T::Output: 'a + std::marker::Send
{
    info!("Starting producer");

    let rec_clone = receiver.clone();
    let mut counter = 0;
    let start_time = SystemTime::now();
    //let mut clone = producer.clone();

    let runtime = tokio::runtime::Runtime::new().expect("Failed to start async tokio runtime");
    runtime.block_on( async {
        let producer_arc= Arc::new(futures::lock::Mutex::new(producer));
        info!("Started async runtime");
        for msg in receiver {
            if msg.is_empty() {
                info!("Empty message received");
                break;
            } else {
                counter+=1;
                if counter%200 == 0{
                    let l = rec_clone.len();
                    PRODUCTION_CHANNEL_QUEUE.set(l as i64);
                }
                let prd = producer_arc.clone();
                let metric = PRODUCTION_LATENCY_BUCKETS.clone();
                tokio::spawn(async move {
                    let chrono = SystemTime::now();
                    prd.lock().await.produce(msg).await;
                    metric.observe(chrono.elapsed().unwrap().as_micros() as f64);
                });
            }
        }
        producer_arc.lock().await.stop();
    });
    println!("Done producing in {} seconds", start_time.elapsed().unwrap().as_secs());
}