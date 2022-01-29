pub mod types;
pub mod random_bytes;

use lazy_static::lazy_static;
use prometheus::IntCounter;

lazy_static! {
    pub static ref GENERATED_MESSAGES_COUNT: IntCounter = IntCounter::new("generated_messages_count", "generated messages count").unwrap();
}