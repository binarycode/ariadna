#[cfg(target_arch = "xtensa")]
use esp_idf_hal::delay::FreeRtos;
use shaku::Component;

#[derive(Component)]
#[shaku(interface = crate::esp32::halt_service::Interface)]
pub struct Service;

impl crate::esp32::halt_service::Interface for Service {
    fn execute(&self) {
        #[cfg(target_arch = "xtensa")]
        {
            loop {
                FreeRtos::delay_ms(u32::MAX);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use shaku::HasComponent;

    shaku::module! {
        TestModule {
            components = [
                crate::esp32::halt_service::Service,
            ],
            providers = []
        }
    }

    #[test]
    fn test_happy_path() {
        let module = TestModule::builder().build();

        HasComponent::<dyn crate::esp32::halt_service::Interface>::resolve(&module).execute();
    }
}
