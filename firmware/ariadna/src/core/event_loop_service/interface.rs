use std::sync::mpsc::Receiver;

pub trait Interface: shaku::Interface {
    fn execute(&self, rx: Receiver<crate::core::Event>) -> Result<(), crate::core::event_loop_service::Error>;
}
