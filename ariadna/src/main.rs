#[cfg(target_arch = "xtensa")]
use display_interface_spi::SPIInterface;
#[cfg(target_arch = "xtensa")]
use embedded_graphics::pixelcolor::Rgb565;
#[cfg(target_arch = "xtensa")]
use embedded_graphics::prelude::*;
#[cfg(target_arch = "xtensa")]
use embedded_graphics::primitives::PrimitiveStyle;
#[cfg(target_arch = "xtensa")]
use embedded_graphics::primitives::Rectangle;
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
use esp_idf_hal::units::FromValueType;
#[cfg(target_arch = "xtensa")]
use mipidsi::Builder;
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
        let sclk = peripherals.pins.gpio12;
        let mosi = peripherals.pins.gpio11;
        let cs = peripherals.pins.gpio10;
        let dc = PinDriver::output(peripherals.pins.gpio9).expect("Failed to init DC pin");
        let rst = PinDriver::output(peripherals.pins.gpio8).expect("Failed to init RST pin");

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
            .orientation(Orientation::new())
            .init(&mut FreeRtos)
            .expect("Failed to init display");

        log::info!("Display initialized, starting color cycle");

        let mut led = Ws2812Esp32RmtDriver::new(peripherals.rmt.channel0, peripherals.pins.gpio38)
            .expect("Failed to init WS2812 LED");
        log::info!("WS2812 LED initialized on GPIO38");

        let colors = [
            (Rgb565::RED, 255, 0, 0),
            (Rgb565::GREEN, 0, 255, 0),
            (Rgb565::BLUE, 0, 0, 255),
        ];
        let color_names = ["RED", "GREEN", "BLUE"];

        let mut idx = 0;
        loop {
            let (display_color, r, g, b) = colors[idx];
            log::info!("Filling display with {}", color_names[idx]);

            let pixel = LedPixelColorGrb24::new_with_rgb(r, g, b);
            led.write_blocking(pixel.as_ref().iter().copied())
                .expect("Failed to write LED");

            Rectangle::new(Point::zero(), display.bounding_box().size)
                .into_styled(PrimitiveStyle::with_fill(display_color))
                .draw(&mut display)
                .expect("Failed to draw");

            idx = (idx + 1) % colors.len();
            FreeRtos::delay_ms(2000);
        }
    }

    #[cfg(not(target_arch = "xtensa"))]
    {
        log::info!("This code is meant to run on ESP32");
    }
}
