use std::sync::Mutex;

use embedded_graphics::Drawable;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::text::Text;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::AnyIOPin;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi::SpiDeviceDriver;
use esp_idf_hal::spi::SpiDriver;
use esp_idf_hal::spi::SpiDriverConfig;
use esp_idf_hal::spi::config::Config as SpiConfig;
use mipidsi::interface::SpiInterface;
use mipidsi::models::ILI9341Rgb565;
use mipidsi::options::ColorOrder;
use mipidsi::options::Orientation;

pub type Spi = esp_idf_hal::spi::SPI2;
pub type SclkPin = esp_idf_hal::gpio::Gpio10;
pub type MosiPin = esp_idf_hal::gpio::Gpio11;
pub type DcPin = esp_idf_hal::gpio::Gpio12;
pub type RstPin = esp_idf_hal::gpio::Gpio13;
pub type CsPin = esp_idf_hal::gpio::Gpio14;

type Display<'a> = mipidsi::Display<
    SpiInterface<'a, SpiDeviceDriver<'a, SpiDriver<'a>>, PinDriver<'a, DcPin, esp_idf_hal::gpio::Output>>,
    ILI9341Rgb565,
    PinDriver<'a, RstPin, esp_idf_hal::gpio::Output>,
>;

#[derive(shaku::Component)]
#[shaku(interface = crate::utils::display_service::Interface)]
pub struct Service {
    #[shaku(default)]
    display: Mutex<Option<Display<'static>>>,
}

impl Service {
    pub fn build_parameters(
        spi: Spi,
        sclk: SclkPin,
        mosi: MosiPin,
        dc: DcPin,
        rst: RstPin,
        cs: CsPin,
    ) -> ServiceParameters {
        let dc = PinDriver::output(dc).unwrap();
        let rst = PinDriver::output(rst).unwrap();

        let config = SpiConfig::new().baudrate(40.MHz().into());
        let driver = SpiDeviceDriver::new_single(
            spi,
            sclk,
            mosi,
            Option::<AnyIOPin>::None,
            Some(cs),
            &SpiDriverConfig::new(),
            &config,
        )
        .unwrap();

        let buffer = Box::leak(Box::new([0u8; 512]));
        let spi_interface = SpiInterface::new(driver, dc, buffer);
        let display = mipidsi::Builder::new(ILI9341Rgb565, spi_interface)
            .reset_pin(rst)
            .orientation(Orientation::new().flip_horizontal())
            .color_order(ColorOrder::Bgr)
            .init(&mut FreeRtos)
            .unwrap();

        ServiceParameters {
            display: Mutex::new(Some(display)),
        }
    }
}

impl crate::utils::display_service::Interface for Service {
    fn clear(&self, color: crate::utils::display_service::Color) -> Result<(), crate::utils::display_service::Error> {
        let mut guard = self
            .display
            .lock()
            .map_err(|_| crate::utils::display_service::Error::PoisonedMutex)?;

        let display = guard
            .as_mut()
            .ok_or(crate::utils::display_service::Error::NotInitialized)?;

        let color = color.to_rgb565();
        display
            .clear(color)
            .map_err(crate::utils::display_service::Error::ClearFailed)?;

        Ok(())
    }

    fn draw_text(
        &self,
        text: &str,
        x: i32,
        y: i32,
        color: crate::utils::display_service::Color,
        background_color: Option<crate::utils::display_service::Color>,
    ) -> Result<(), crate::utils::display_service::Error> {
        let mut guard = self
            .display
            .lock()
            .map_err(|_| crate::utils::display_service::Error::PoisonedMutex)?;

        let display = guard
            .as_mut()
            .ok_or(crate::utils::display_service::Error::NotInitialized)?;

        let color = color.to_rgb565();
        let mut style = MonoTextStyle::new(&FONT_6X10, color);
        style.background_color = background_color.map(|c| c.to_rgb565());

        Text::new(text, Point::new(x, y), style)
            .draw(display)
            .map_err(crate::utils::display_service::Error::DrawTextFailed)?;

        Ok(())
    }

    fn size(&self) -> (u32, u32) {
        (240, 320)
    }
}
