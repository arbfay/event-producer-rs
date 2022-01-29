pub trait Generate {
    fn generate(&self) -> Vec<u8>;
}

pub trait GeneratorLoop {
    fn run_generation_loop(&self, sender: crossbeam_channel::Sender<Vec<u8>>);
}

