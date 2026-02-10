pub trait Esp32ServiceInterface: shaku::Interface {
    fn init(&self);
    fn halt(&self);
}

#[derive(shaku::Component)]
#[shaku(interface = Esp32ServiceInterface)]
pub struct Esp32Service;

impl Esp32ServiceInterface for Esp32Service {
    fn init(&self) {
        #[cfg(target_arch = "xtensa")]
        {
            esp_idf_svc::sys::link_patches();
            esp_idf_svc::log::EspLogger::initialize_default();
        }
    }

    fn halt(&self) {
        #[cfg(target_arch = "xtensa")]
        {
            loop {
                esp_idf_hal::delay::FreeRtos::delay_ms(u32::MAX);
            }
        }
    }
}
