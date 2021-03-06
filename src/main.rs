use prometheus::{Registry, Encoder};
use crossbeam_channel::{unbounded};
use crossbeam_utils::thread;
use event_producer::{
    self, 
    generator::{random_bytes::RandomGenerator, types::GeneratorLoop, GENERATED_MESSAGES_COUNT}, 
    producer::{stdout::StdoutProducer, PRODUCTION_MESSAGES_SENT, PRODUCTION_CHANNEL_QUEUE}};

fn main() {
    pretty_env_logger::init_timed();
    
    let settings = event_producer::settings::Settings::new().unwrap();
    
    // Prometheus metrics registry
    let metrics_registry = Registry::new_custom(Some("random_producer".to_string()), None).unwrap();
    metrics_registry.register(Box::new(GENERATED_MESSAGES_COUNT.clone())).expect("Failed to register metric");
    metrics_registry.register(Box::new(PRODUCTION_MESSAGES_SENT.clone())).expect("Failed to register metric");
    metrics_registry.register(Box::new(PRODUCTION_CHANNEL_QUEUE.clone())).expect("Failed to register metric");
    
    // Create channel
    let (sender, receiver) = unbounded();

    // Create 3 threads - 1 for generation, 1 for production, 1 for the metrics server
    thread::scope(|scope|{

        // Launch generator with sender
        scope.builder()
            .name("generation".to_string())
            .spawn(move |_| {
                // Create generator
                let generator = RandomGenerator::new(settings.generator.random.unwrap());
                generator.run_generation_loop(sender);
            })
            .expect("Failed to spawn thread: generation");


        // Launch producer with receiver
        scope.builder()
            .name("production".to_string())
            .spawn(move |_| {
                // Create producer
                if settings.producer.stdout.is_some() {
                    let producer = StdoutProducer::new(settings.producer.stdout.unwrap());
                    event_producer::producer::run_production_loop(producer, receiver);
                } else if settings.producer.kafka.is_some() & cfg!(feature = "kafka") {
                    let producer = event_producer::producer::kafka::KafkaProducer::new(settings.producer.kafka.unwrap());
                    event_producer::producer::run_production_loop(producer, receiver);
                }
                //producer.run_production_loop(receiver);
            })
            .expect("Failed to spawn thread: generation");
        
        // Launch metrics server if enabled
        if settings.metrics.enable{
            scope.builder()
                .name("metrics server".to_string())
                .spawn(|_| {
                    event_producer::metrics::start_metrics_service(settings.metrics, Box::new(metrics_registry.clone()))
                })
                .expect("Failed to start metrics server");
        }
    })
    .unwrap();

    let metrics = metrics_registry.gather();
    let encoder = prometheus::TextEncoder::new();
    let mut buffer = vec![];
    encoder.encode(&metrics, &mut buffer).unwrap();
    println!("{}", String::from_utf8(buffer.to_ascii_lowercase()).unwrap());
}