use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

pub enum ScaleCommand {}
pub enum ScaleResponse {}
enum ScaleMode {
    test
}

pub struct ScaleThread {
    receiver: Receiver<ScaleCommand>,
    sender: Sender<ScaleResponse>,
    mode: ScaleMode,
}

impl ScaleThread{
    pub fn new (receiver:Receiver<ScaleCommand>,sender:Sender<ScaleResponse>) -> Self{
        Self {
            receiver : receiver,
            sender : sender,
            mode : ScaleMode::test
        }
    }
    pub async fn run(&self) -> Option<> {
        loop {
            // rxとtxを使った処理
        }
        Ok()
    }
}
