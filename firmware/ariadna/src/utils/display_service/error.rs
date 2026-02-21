#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum Error {
    #[cfg(target_arch = "xtensa")]
    #[error("Poisoned Mutex")]
    PoisonedMutex,

    #[error("Display is not initialized")]
    NotInitialized,

    #[cfg(target_arch = "xtensa")]
    #[error("Failed to clear display: {0:?}")]
    ClearFailed(mipidsi::interface::SpiError<esp_idf_hal::spi::SpiError, esp_idf_hal::gpio::GpioError>),

    #[cfg(target_arch = "xtensa")]
    #[error("Failed to draw text: {0:?}")]
    DrawTextFailed(mipidsi::interface::SpiError<esp_idf_hal::spi::SpiError, esp_idf_hal::gpio::GpioError>),
}
