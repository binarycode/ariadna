use std::thread;
use std::time::Duration;

#[cfg(target_arch = "xtensa")]
use esp_idf_hal::gpio::PinDriver;
#[cfg(target_arch = "xtensa")]
use esp_idf_hal::peripherals::Peripherals;

fn main() {
    #[cfg(target_arch = "xtensa")]
    {
        esp_idf_svc::sys::link_patches();
        esp_idf_svc::log::EspLogger::initialize_default();

        log::info!("Starting LED blink demo");

        let peripherals = Peripherals::take().expect("Failed to take peripherals");
        let mut led = PinDriver::output(peripherals.pins.gpio4).expect("Failed to init LED pin");

        let mut on = false;
        loop {
            on = !on;
            if on {
                log::info!("LED ON");
                led.set_high().unwrap();
            } else {
                log::info!("LED OFF");
                led.set_low().unwrap();
            }
            thread::sleep(Duration::from_millis(500));
        }
    }

    #[cfg(not(target_arch = "xtensa"))]
    {
        log::info!("This code is meant to run on ESP32");
    }
}
