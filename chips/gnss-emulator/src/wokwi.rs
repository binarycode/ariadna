#[cfg(target_arch = "wasm32")]
use std::ffi::CString;

pub use wokwi_chip_ll::INPUT;
pub use wokwi_chip_ll::OUTPUT;
pub use wokwi_chip_ll::PinId;
pub use wokwi_chip_ll::TimerConfig;
pub use wokwi_chip_ll::TimerId;
pub use wokwi_chip_ll::UARTConfig;
pub use wokwi_chip_ll::UARTDevId;

#[cfg(not(target_arch = "wasm32"))]
mockall::mock! {
    pub Api {
        pub fn debug_print(&self, message: &str);
        pub fn pin_init(&self, name: &str, mode: u32) -> PinId;
        pub fn timer_init(&self, config: &TimerConfig) -> TimerId;
        pub fn timer_start(&self, timer_id: TimerId, ms: u32, repeat: bool);
        pub fn uart_init(&self, config: &UARTConfig) -> UARTDevId;
        pub fn uart_write(&self, uart_id: UARTDevId, buffer: &[u8]) -> bool;
    }
}

#[cfg(not(target_arch = "wasm32"))]
thread_local! {
    pub static MOCK_API: std::cell::RefCell<MockApi> = std::cell::RefCell::new(MockApi::default());
}

pub fn debug_print(message: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        let message = CString::new(message).unwrap();
        let message = message.as_ptr();
        unsafe { wokwi_chip_ll::debugPrint(message) }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        MOCK_API.with(|m| m.borrow().debug_print(message))
    }
}

pub fn pin_init(name: &str, mode: u32) -> PinId {
    #[cfg(target_arch = "wasm32")]
    {
        let name = CString::new(name).unwrap();
        let name = name.as_ptr();
        unsafe { wokwi_chip_ll::pinInit(name, mode) }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        MOCK_API.with(|m| m.borrow().pin_init(name, mode))
    }
}

pub fn timer_init(config: &TimerConfig) -> TimerId {
    #[cfg(target_arch = "wasm32")]
    {
        unsafe { wokwi_chip_ll::timerInit(config) }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        MOCK_API.with(|m| m.borrow().timer_init(config))
    }
}

pub fn timer_start(timer_id: TimerId, ms: u32, repeat: bool) {
    #[cfg(target_arch = "wasm32")]
    {
        unsafe { wokwi_chip_ll::timerStart(timer_id, ms, repeat) }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        MOCK_API.with(|m| m.borrow().timer_start(timer_id, ms, repeat))
    }
}

pub fn uart_init(config: &UARTConfig) -> UARTDevId {
    #[cfg(target_arch = "wasm32")]
    {
        unsafe { wokwi_chip_ll::uartInit(config) }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        MOCK_API.with(|m| m.borrow().uart_init(config))
    }
}

pub fn uart_write(uart_id: UARTDevId, buffer: &[u8]) -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        let count = buffer.len() as u32;
        let buffer = buffer.as_ptr();
        unsafe { wokwi_chip_ll::uartWrite(uart_id, buffer, count) }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        MOCK_API.with(|m| m.borrow().uart_write(uart_id, buffer))
    }
}
