use std::io::BufRead;
use std::io::BufReader;
use std::sync::Mutex;

use embedded_io_adapters::std::ToStd;
use esp_idf_hal::gpio::AnyIOPin;
use esp_idf_hal::prelude::*;
use esp_idf_hal::uart::UartDriver;
use esp_idf_hal::uart::config::Config as UartConfig;
use shaku::Component;

#[derive(Component)]
#[shaku(interface = crate::gnss::uart_service::Interface)]
pub struct Service {
    #[shaku(default)]
    reader: Mutex<Option<BufReader<ToStd<UartDriver<'static>>>>>,
}

pub type Uart = esp_idf_hal::uart::UART1;
pub type TxPin = esp_idf_hal::gpio::Gpio43;
pub type RxPin = esp_idf_hal::gpio::Gpio44;

impl Service {
    pub fn build_parameters(uart: Uart, tx: TxPin, rx: RxPin) -> ServiceParameters {
        let config = UartConfig::default().baudrate(9600.Hz());
        let driver = UartDriver::new(
            uart,
            tx,
            rx,
            Option::<AnyIOPin>::None,
            Option::<AnyIOPin>::None,
            &config,
        )
        .unwrap();
        let reader = BufReader::new(ToStd::new(driver));

        ServiceParameters {
            reader: Mutex::new(Some(reader)),
        }
    }
}

impl crate::gnss::uart_service::Interface for Service {
    fn read_line(&self) -> Result<String, crate::gnss::uart_service::Error> {
        let mut guard = self
            .reader
            .lock()
            .map_err(|_| crate::gnss::uart_service::Error::PoisonedMutex)?;

        let reader = guard.as_mut().ok_or(crate::gnss::uart_service::Error::NotInitialized)?;

        let mut line = String::new();
        reader
            .read_line(&mut line)
            .map_err(crate::gnss::uart_service::Error::ReadFailure)?;

        Ok(line)
    }
}
