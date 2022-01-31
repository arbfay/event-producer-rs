use async_trait::async_trait;
use futures::Future;

#[async_trait]
pub trait Produce {
    type Output: Future<Output = Result<(), String>>;
    async fn produce(&mut self, message: Vec<u8>) -> Self::Output;
    fn stop(&self) {}
}

pub type ImplOutput = std::future::Ready<Result<(), String>>;

pub trait ProductionLoop {
    fn run_production_loop(&self, receiver: crossbeam_channel::Receiver<Vec<u8>>);
}
