use self::types::Produce;
use lazy_static::lazy_static;
use crossbeam_channel::Receiver;
use prometheus::{IntGauge, IntCounter};

#[cfg(feature = "kafka")]
pub mod kafka;

pub mod stdout;
pub mod types;

lazy_static! {
    pub static ref PRODUCTION_CHANNEL_QUEUE: IntGauge = IntGauge::new("production_channel_queue_size", "events waiting to be processed").unwrap();
    pub static ref PRODUCTION_MESSAGES_SENT: IntCounter = IntCounter::new("production_messages_sent", "message sent to cluster").unwrap();
    pub static ref PRODUCTION_MESSAGES_FAILED_TO_SEND: IntCounter = IntCounter::new("production_messages_failed_to_send", "message failed to be sent to cluster").unwrap();
}

pub fn run_production_loop<T: Produce>(
    producer: T,
    receiver: Receiver<Vec<u8>>
){
    let rec_clone = receiver.clone();
    let mut counter = 0;
    for msg in receiver {
        if msg.is_empty() {
            break;
        } else {
            counter+=1;
            producer.produce(msg).unwrap();
        }
        if counter%200 == 0{
            let l = rec_clone.len();
            PRODUCTION_CHANNEL_QUEUE.set(l as i64);
        }
    }
    producer.stop();
}