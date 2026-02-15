#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum Error {
    #[cfg(target_arch = "xtensa")]
    #[error("Poisoned Mutex")]
    PoisonedMutex,

    #[cfg(target_arch = "xtensa")]
    #[error("Failed to initialize UART driver: {0:?}")]
    UartInitializationFailure(esp_idf_sys::EspError),

    #[error("UART is not initialized")]
    NotInitialized,

    #[cfg(target_arch = "xtensa")]
    #[error("Failed to read from UART: {0:?}")]
    ReadFailure(std::io::Error),
}
