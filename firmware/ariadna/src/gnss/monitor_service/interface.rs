use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

pub trait Interface: shaku::Interface {
    fn execute(&self, tx: Sender<crate::core::Event>) -> JoinHandle<()>;
}
