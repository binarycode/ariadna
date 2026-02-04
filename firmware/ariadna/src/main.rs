#[cfg(target_arch = "xtensa")]
use std::thread;

#[cfg(target_arch = "xtensa")]
use display_interface_spi::SPIInterface;
#[cfg(target_arch = "xtensa")]
use embedded_graphics::image::ImageRaw;
#[cfg(target_arch = "xtensa")]
use embedded_graphics::image::ImageRawLE;
#[cfg(target_arch = "xtensa")]
use embedded_graphics::pixelcolor::Rgb565;
#[cfg(target_arch = "xtensa")]
use embedded_graphics::prelude::*;
#[cfg(target_arch = "xtensa")]
use esp_idf_hal::delay::FreeRtos;
#[cfg(target_arch = "xtensa")]
use esp_idf_hal::gpio::PinDriver;
#[cfg(target_arch = "xtensa")]
use esp_idf_hal::peripherals::Peripherals;
#[cfg(target_arch = "xtensa")]
use esp_idf_hal::spi::SpiDeviceDriver;
#[cfg(target_arch = "xtensa")]
use esp_idf_hal::spi::SpiDriverConfig;
#[cfg(target_arch = "xtensa")]
use esp_idf_hal::spi::config::Config;
#[cfg(target_arch = "xtensa")]
use esp_idf_hal::uart::UartDriver;
#[cfg(target_arch = "xtensa")]
use esp_idf_hal::uart::config::Config as UartConfig;
#[cfg(target_arch = "xtensa")]
use esp_idf_hal::units::FromValueType;
#[cfg(target_arch = "xtensa")]
use mipidsi::Builder;
#[cfg(target_arch = "xtensa")]
use mipidsi::options::ColorOrder;
#[cfg(target_arch = "xtensa")]
use mipidsi::options::Orientation;
#[cfg(target_arch = "xtensa")]
use ws2812_esp32_rmt_driver::driver::Ws2812Esp32RmtDriver;
#[cfg(target_arch = "xtensa")]
use ws2812_esp32_rmt_driver::driver::color::LedPixelColor;
#[cfg(target_arch = "xtensa")]
use ws2812_esp32_rmt_driver::driver::color::LedPixelColorGrb24;

fn main() {
    #[cfg(target_arch = "xtensa")]
    {
        esp_idf_svc::sys::link_patches();
        esp_idf_svc::log::EspLogger::initialize_default();

        log::info!("Starting ILI9341 display demo");

        let peripherals = Peripherals::take().expect("Failed to take peripherals");

        // SPI pins for ILI9341
        let sclk = peripherals.pins.gpio10;
        let mosi = peripherals.pins.gpio11;
        let cs = peripherals.pins.gpio14;
        let dc = PinDriver::output(peripherals.pins.gpio12).expect("Failed to init DC pin");
        let rst = PinDriver::output(peripherals.pins.gpio13).expect("Failed to init RST pin");

        // Configure SPI
        let spi = SpiDeviceDriver::new_single(
            peripherals.spi2,
            sclk,
            mosi,
            None::<esp_idf_hal::gpio::AnyIOPin>,
            Some(cs),
            &SpiDriverConfig::new(),
            &Config::new().baudrate(40.MHz().into()),
        )
        .expect("Failed to init SPI");

        // Create display interface
        let di = SPIInterface::new(spi, dc);

        // Initialize display
        let mut display = Builder::new(mipidsi::models::ILI9341Rgb565, di)
            .reset_pin(rst)
            .orientation(Orientation::new().flip_horizontal())
            .color_order(ColorOrder::Bgr)
            .init(&mut FreeRtos)
            .expect("Failed to init display");

        log::info!("Display initialized");

        let mut led = Ws2812Esp32RmtDriver::new(peripherals.rmt.channel0, peripherals.pins.gpio38)
            .expect("Failed to init WS2812 LED");
        log::info!("WS2812 LED initialized on GPIO38");

        let gnss_uart = UartDriver::new(
            peripherals.uart1,
            peripherals.pins.gpio43,
            peripherals.pins.gpio44,
            Option::<esp_idf_hal::gpio::AnyIOPin>::None,
            Option::<esp_idf_hal::gpio::AnyIOPin>::None,
            &UartConfig::default().baudrate(9600.Hz()),
        )
        .expect("Failed to init GNSS UART");
        log::info!("GNSS UART initialized on GPIO43 (TX) and GPIO44 (RX)");

        thread::Builder::new()
            .stack_size(4096)
            .spawn(move || {
                let mut buf = [0u8; 256];
                loop {
                    match gnss_uart.read(&mut buf, 100) {
                        Ok(len) if len > 0 => {
                            if let Ok(msg) = std::str::from_utf8(&buf[..len]) {
                                for line in msg.lines() {
                                    if !line.is_empty() {
                                        log::info!("GNSS: {}", line);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            })
            .expect("Failed to spawn GNSS task");

        // Blue light before drawing
        let blue = LedPixelColorGrb24::new_with_rgb(0, 0, 255);
        led.write_blocking(blue.as_ref().iter().copied())
            .expect("Failed to write LED");
        log::info!("Drawing map...");

        // Load and draw the RGB565 map data (240x320, little-endian)
        const MAP_DATA: &[u8] = include_bytes!("../map.bin");
        let raw_image: ImageRawLE<Rgb565> = ImageRaw::new(MAP_DATA, 240);

        raw_image.draw(&mut display).expect("Failed to draw map");

        // Green light after drawing
        let green = LedPixelColorGrb24::new_with_rgb(0, 255, 0);
        led.write_blocking(green.as_ref().iter().copied())
            .expect("Failed to write LED");
        log::info!("Map displayed");

        loop {
            FreeRtos::delay_ms(1000);
        }
    }

    #[cfg(not(target_arch = "xtensa"))]
    {
        log::info!("This code is meant to run on ESP32");
    }
}
