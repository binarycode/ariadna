mod wokwi;

use std::cell::RefCell;
use std::ffi::c_void;

const UART_BAUD_RATE: u32 = 9600;
const PERIOD_US: u32 = 1_000_000;

// NMEA 0183 standard message:
// $GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47
// Fixed position: 4807.038,N, 01131.000,E
const NMEA_MSG: &[u8] = b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";

struct Chip {
    uart_id: wokwi::UARTDevId,
}

thread_local! {
    static CHIPS: RefCell<Vec<Chip>> = const { RefCell::new(Vec::new()) };
}

#[unsafe(export_name = "chipInit")]
pub extern "C" fn chip_init() {
    wokwi::debug_print("Initializing GNSS Emulator...");

    let index = CHIPS.with(|chips| chips.borrow().len());

    let rx_pin = wokwi::pin_init("RX", wokwi::INPUT);
    let tx_pin = wokwi::pin_init("TX", wokwi::OUTPUT);

    let uart_id = wokwi::uart_init(&wokwi::UARTConfig {
        user_data: index as *const c_void,
        rx: rx_pin,
        tx: tx_pin,
        baud_rate: UART_BAUD_RATE,
        rx_data: on_uart_rx_data,
        write_done: on_uart_write_done,
    });
    CHIPS.with(|chips| {
        chips.borrow_mut().push(Chip { uart_id });
    });

    let timer_id = wokwi::timer_init(&wokwi::TimerConfig {
        user_data: index as *const c_void,
        callback: on_msg_timer,
    });
    wokwi::timer_start(timer_id, PERIOD_US, true);
}

extern "C" fn on_msg_timer(user_data: *mut c_void) {
    CHIPS.with(|chips| {
        let chips = chips.borrow();
        if let Some(chip) = chips.get(user_data as usize) {
            wokwi::uart_write(chip.uart_id, NMEA_MSG);
        }
    });
}

extern "C" fn on_uart_rx_data(_user_data: *mut c_void, _byte: u8) {}

extern "C" fn on_uart_write_done(_user_data: *mut c_void) {}

#[cfg(test)]
mod tests {
    use std::ffi::c_void;
    use std::ptr::fn_addr_eq;

    use mockall::predicate::*;

    use crate::wokwi;

    #[test]
    fn test_chip_init() {
        crate::CHIPS.with(|chips| chips.borrow_mut().clear());

        let mut mock = wokwi::MockApi::default();

        mock.expect_debug_print()
            .with(eq("Initializing GNSS Emulator..."))
            .times(1)
            .returning(|_| ());

        mock.expect_pin_init()
            .with(eq("RX"), eq(wokwi::INPUT))
            .times(1)
            .returning(|_, _| 10);

        mock.expect_pin_init()
            .with(eq("TX"), eq(wokwi::OUTPUT))
            .times(1)
            .returning(|_, _| 11);

        mock.expect_uart_init()
            .withf(|config| {
                let expected_config = wokwi::UARTConfig {
                    user_data: 0 as *const c_void,
                    rx: 10,
                    tx: 11,
                    baud_rate: crate::UART_BAUD_RATE,
                    rx_data: crate::on_uart_rx_data,
                    write_done: crate::on_uart_write_done,
                };

                config.user_data == expected_config.user_data
                    && config.rx == expected_config.rx
                    && config.tx == expected_config.tx
                    && config.baud_rate == expected_config.baud_rate
                    && fn_addr_eq(config.rx_data, expected_config.rx_data)
                    && fn_addr_eq(config.write_done, expected_config.write_done)
            })
            .times(1)
            .returning(|_| 20);

        mock.expect_timer_init()
            .withf(|config| {
                let expected_config = wokwi::TimerConfig {
                    user_data: 0 as *const c_void,
                    callback: crate::on_msg_timer,
                };

                config.user_data == expected_config.user_data && fn_addr_eq(config.callback, expected_config.callback)
            })
            .times(1)
            .returning(|_| 30);

        mock.expect_timer_start()
            .with(eq(30), eq(crate::PERIOD_US), eq(true))
            .times(1)
            .returning(|_, _, _| ());

        wokwi::MOCK_API.with(|m| *m.borrow_mut() = mock);

        crate::chip_init();
    }

    #[test]
    fn test_on_msg_timer() {
        crate::CHIPS.with(|chips| {
            chips.borrow_mut().clear();
            chips.borrow_mut().push(crate::Chip { uart_id: 42 });
        });

        let mut mock = wokwi::MockApi::default();

        mock.expect_uart_write()
            .with(eq(42), eq(crate::NMEA_MSG))
            .times(1)
            .returning(|_, _| true);

        wokwi::MOCK_API.with(|m| *m.borrow_mut() = mock);

        crate::on_msg_timer(0 as *mut c_void);
    }
}
