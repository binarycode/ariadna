use shaku::Component;

#[derive(Component)]
#[shaku(interface = crate::esp32::initialize_service::Interface)]
pub struct Service;

impl crate::esp32::initialize_service::Interface for Service {
    fn execute(&self) {
        #[cfg(target_arch = "xtensa")]
        {
            esp_idf_svc::sys::link_patches();
            esp_idf_svc::log::EspLogger::initialize_default();
        }
    }
}

#[cfg(test)]
mod tests {
    use shaku::HasComponent;

    shaku::module! {
        TestModule {
            components = [
                crate::esp32::initialize_service::Service,
            ],
            providers = []
        }
    }

    #[test]
    fn test_happy_path() {
        let module = TestModule::builder().build();

        HasComponent::<dyn crate::esp32::initialize_service::Interface>::resolve(&module).execute();
    }
}
