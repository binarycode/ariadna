pub trait Interface: shaku::Interface {
    fn read_line(&self) -> Result<String, crate::gnss::uart_service::Error>;
}
