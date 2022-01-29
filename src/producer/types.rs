pub trait Produce {
    fn produce(&self, message: Vec<u8>) -> Result<(), String>;
    fn stop(&self) {}
}

pub trait ProductionLoop {
    fn run_production_loop(&self, receiver: crossbeam_channel::Receiver<Vec<u8>>);
}
