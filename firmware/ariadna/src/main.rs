#[cfg(target_arch = "xtensa")]
use esp_idf_hal::delay::FreeRtos;

fn main() {
    #[cfg(target_arch = "xtensa")]
    {
        esp_idf_svc::sys::link_patches();
        esp_idf_svc::log::EspLogger::initialize_default();

        loop {
            FreeRtos::delay_ms(1000);
        }
    }

    #[cfg(not(target_arch = "xtensa"))]
    {
        log::info!("This code is meant to run on ESP32");
    }
}
