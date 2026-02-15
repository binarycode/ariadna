pub trait Interface: shaku::Interface {
    fn execute(&self) -> Result<(), crate::core::main_service::Error>;
}
