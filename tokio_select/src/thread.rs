use tokio::sync::mpsc::{Receiver, Sender};

pub enum Command {}
pub enum Response {}
enum Mode {
    test
}

pub struct Thread {
    receiver: Receiver<ScaleCommand>,
    sender: Sender<ScaleResponse>,
    mode: ScaleMode,
}

impl Thread{
    pub fn new (receiver:Receiver<Command>,sender:Sender<Response>) -> Self{
        Self {
            receiver : receiver,
            sender : sender,
            mode : Mode::test
        }
    }
    pub async fn run(&self) -> Option<> {
        loop {
            // rxとtxを使った処理
        }
        Ok()
    }
}
