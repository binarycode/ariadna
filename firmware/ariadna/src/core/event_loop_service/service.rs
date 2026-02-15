use std::sync::Arc;
use std::sync::mpsc::Receiver;

use shaku::Component;

#[derive(Component)]
#[shaku(interface = crate::core::event_loop_service::Interface)]
pub struct Service {
    #[shaku(inject)]
    utils_introspect_state_service: Arc<dyn crate::utils::introspect_state_service::Interface>,
}

impl crate::core::event_loop_service::Interface for Service {
    fn execute(&self, rx: Receiver<crate::core::Event>) -> Result<(), crate::core::event_loop_service::Error> {
        let mut state = crate::core::State {
            latitude: None,
            longitude: None,
        };

        loop {
            match rx.recv() {
                Ok(crate::core::Event::Halt) => {
                    break;
                }
                Ok(crate::core::Event::GnssFix { latitude, longitude }) => {
                    state.latitude = Some(latitude);
                    state.longitude = Some(longitude);
                }
                Ok(crate::core::Event::IntrospectState) => {
                    self.utils_introspect_state_service.execute(&state);
                }
                #[allow(unreachable_patterns)]
                Ok(_) => {}
                Err(e) => {
                    return Err(crate::core::event_loop_service::Error::ReceiveError(e));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;

    use shaku::HasComponent;

    shaku::module! {
        TestModule {
            components = [
                crate::core::event_loop_service::Service,
                crate::utils::introspect_state_service::MockService,
            ],
            providers = []
        }
    }

    #[test]
    fn test_halt_terminates_loop() {
        let module = TestModule::builder().build();

        let (tx, rx) = channel();
        tx.send(crate::core::Event::Halt).unwrap();

        HasComponent::<dyn crate::core::event_loop_service::Interface>::resolve(&module)
            .execute(rx)
            .unwrap();
    }

    #[test]
    fn test_receive_error_terminates_loop() {
        let module = TestModule::builder().build();

        let (tx, rx) = channel();
        drop(tx);

        let result = HasComponent::<dyn crate::core::event_loop_service::Interface>::resolve(&module).execute(rx);
        assert_eq!(
            result,
            Err(crate::core::event_loop_service::Error::ReceiveError(
                std::sync::mpsc::RecvError
            ))
        );
    }

    #[test]
    fn test_gnss_fix_updates_state() {
        let mut mock_utils_introspect_state_service = crate::utils::introspect_state_service::MockService::new();

        let mut seq = mockall::Sequence::new();
        mock_utils_introspect_state_service
            .expect_execute()
            .withf(|state| state.latitude.is_none() && state.longitude.is_none())
            .times(1)
            .in_sequence(&mut seq);
        mock_utils_introspect_state_service
            .expect_execute()
            .withf(|state| state.latitude == Some(20.5) && state.longitude == Some(10.5))
            .times(1)
            .in_sequence(&mut seq);

        let module = TestModule::builder()
            .with_component_override::<dyn crate::utils::introspect_state_service::Interface>(Box::new(
                mock_utils_introspect_state_service,
            ))
            .build();

        let (tx, rx) = channel();
        tx.send(crate::core::Event::IntrospectState).unwrap();
        tx.send(crate::core::Event::GnssFix {
            latitude: 20.5,
            longitude: 10.5,
        })
        .unwrap();
        tx.send(crate::core::Event::IntrospectState).unwrap();
        tx.send(crate::core::Event::Halt).unwrap();

        HasComponent::<dyn crate::core::event_loop_service::Interface>::resolve(&module)
            .execute(rx)
            .unwrap();
    }
}
