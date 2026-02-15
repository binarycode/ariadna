use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

use nmea::ParseResult;
use shaku::Component;

#[derive(Component)]
#[shaku(interface = crate::gnss::monitor_service::Interface)]
pub struct Service {
    #[shaku(inject)]
    uart_service: Arc<dyn crate::gnss::uart_service::Interface>,
}

impl crate::gnss::monitor_service::Interface for Service {
    fn execute(&self, tx: Sender<crate::core::Event>) -> JoinHandle<()> {
        let uart_service = self.uart_service.clone();

        thread::spawn(move || {
            loop {
                match uart_service.read_line() {
                    Ok(line) => match nmea::parse_str(line.trim_end()) {
                        Ok(ParseResult::GGA(gga)) => match (gga.latitude, gga.longitude) {
                            (Some(latitude), Some(longitude)) => {
                                if let Err(e) = tx.send(crate::core::Event::GnssFix { latitude, longitude }) {
                                    log::error!("Failed to send GNSS fix event: {e:?}");
                                    return;
                                }
                            }
                            _ => {
                                log::warn!("Missing coordinates in GGA sentence: {gga:?}");
                            }
                        },
                        Ok(_) => {}
                        Err(e) => {
                            log::warn!("NMEA sentence parse error: {e:?}");
                        }
                    },
                    Err(e) => {
                        log::error!("UART read error: {e:?}");
                        return;
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;

    use shaku::HasComponent;

    shaku::module! {
        TestModule {
            components = [
                crate::gnss::monitor_service::Service,
                crate::gnss::uart_service::MockService,
            ],
            providers = []
        }
    }

    #[test]
    fn test_happy_path_sends_gnss_fix_event() {
        let mut mock_gnss_uart_service = crate::gnss::uart_service::MockService::new();

        let mut seq = mockall::Sequence::new();
        mock_gnss_uart_service
            .expect_read_line()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok("$GPGGA,123519.00,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*69".to_string()));
        mock_gnss_uart_service
            .expect_read_line()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Err(crate::gnss::uart_service::Error::NotInitialized));

        let module = TestModule::builder()
            .with_component_override::<dyn crate::gnss::uart_service::Interface>(Box::new(mock_gnss_uart_service))
            .build();

        let (tx, rx) = channel();

        let handle = HasComponent::<dyn crate::gnss::monitor_service::Interface>::resolve(&module).execute(tx);

        let event = rx.recv().unwrap();
        match event {
            crate::core::Event::GnssFix { latitude, longitude } => {
                assert!((latitude - 48.1173).abs() < 0.001);
                assert!((longitude - 11.5166).abs() < 0.001);
            }
            _ => panic!("Expected GnssFix event"),
        }

        handle.join().unwrap();
    }

    #[test]
    fn test_intermittent_problems_do_not_break_monitoring() {
        let mut mock_gnss_uart_service = crate::gnss::uart_service::MockService::new();

        let mut seq = mockall::Sequence::new();
        // First call: normal GPGGA data
        mock_gnss_uart_service
            .expect_read_line()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok("$GPGGA,123519.00,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*69".to_string()));
        // Second call: GPGGA without coordinates
        mock_gnss_uart_service
            .expect_read_line()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok("$GPGGA,123519.00,,,,,0,00,99.99,,,,,,*6B".to_string()));
        // Third call: invalid NMEA sentence
        mock_gnss_uart_service
            .expect_read_line()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok("garbage data".to_string()));
        // Fourth call: normal GPGGA data again (different coordinates)
        mock_gnss_uart_service
            .expect_read_line()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok("$GPGGA,123520.00,5000.000,N,01000.000,E,1,08,0.9,545.4,M,46.9,M,,*65".to_string()));
        // Fifth call: error to break the loop
        mock_gnss_uart_service
            .expect_read_line()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Err(crate::gnss::uart_service::Error::NotInitialized));

        let module = TestModule::builder()
            .with_component_override::<dyn crate::gnss::uart_service::Interface>(Box::new(mock_gnss_uart_service))
            .build();

        let (tx, rx) = channel();

        let handle = HasComponent::<dyn crate::gnss::monitor_service::Interface>::resolve(&module).execute(tx);

        // First event from first read_line
        let event1 = rx.recv().unwrap();
        match event1 {
            crate::core::Event::GnssFix { latitude, longitude } => {
                assert!((latitude - 48.1173).abs() < 0.001);
                assert!((longitude - 11.5166).abs() < 0.001);
            }
            _ => panic!("Expected GnssFix event for first read"),
        }

        // Second event from fourth read_line
        let event2 = rx.recv().unwrap();
        match event2 {
            crate::core::Event::GnssFix { latitude, longitude } => {
                assert!((latitude - 50.0).abs() < 0.001);
                assert!((longitude - 10.0).abs() < 0.001);
            }
            _ => panic!("Expected GnssFix event for fourth read"),
        }

        handle.join().unwrap();
    }
}
