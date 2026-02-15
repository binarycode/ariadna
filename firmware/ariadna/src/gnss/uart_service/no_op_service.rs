use shaku::Component;

#[derive(Component)]
#[shaku(interface = crate::gnss::uart_service::Interface)]
pub struct Service;

impl crate::gnss::uart_service::Interface for Service {
    fn read_line(&self) -> Result<String, crate::gnss::uart_service::Error> {
        Err(crate::gnss::uart_service::Error::NotInitialized)
    }
}
