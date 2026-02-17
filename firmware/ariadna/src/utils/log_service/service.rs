use shaku::Component;

#[derive(Component)]
#[shaku(interface = crate::utils::log_service::Interface)]
pub struct Service;

impl crate::utils::log_service::Interface for Service {
    fn trace(&self, message: &str) {
        log::trace!("{message}");
    }

    fn debug(&self, message: &str) {
        log::debug!("{message}");
    }

    fn info(&self, message: &str) {
        log::info!("{message}");
    }

    fn warn(&self, message: &str) {
        log::warn!("{message}");
    }

    fn error(&self, message: &str) {
        log::error!("{message}");
    }
}
