use std::sync::Arc;
use std::sync::mpsc::channel;

use shaku::Component;

#[derive(Component)]
#[shaku(interface = crate::core::main_service::Interface)]
pub struct Service {
    #[shaku(inject)]
    core_event_loop_service: Arc<dyn crate::core::event_loop_service::Interface>,
    #[shaku(inject)]
    esp32_halt_service: Arc<dyn crate::esp32::halt_service::Interface>,
    #[shaku(inject)]
    esp32_initialize_service: Arc<dyn crate::esp32::initialize_service::Interface>,
    #[shaku(inject)]
    gnss_monitor_service: Arc<dyn crate::gnss::monitor_service::Interface>,
}

impl crate::core::main_service::Interface for Service {
    fn execute(&self) -> Result<(), crate::core::main_service::Error> {
        self.esp32_initialize_service.execute();

        let (tx, rx) = channel::<crate::core::Event>();

        self.gnss_monitor_service.execute(tx);

        let result = self.core_event_loop_service.execute(rx);

        self.esp32_halt_service.execute();

        result.map_err(crate::core::main_service::Error::from)
    }
}

#[cfg(test)]
mod tests {
    use shaku::HasComponent;

    shaku::module! {
        TestModule {
            components = [
                crate::esp32::halt_service::MockService,
                crate::esp32::initialize_service::MockService,
                crate::core::event_loop_service::MockService,
                crate::core::main_service::Service,
                crate::gnss::monitor_service::MockService,
            ],
            providers = []
        }
    }

    #[test]
    fn test_happy_path() {
        let mut mock_core_event_loop_service = crate::core::event_loop_service::MockService::new();
        let mut mock_esp32_halt_service = crate::esp32::halt_service::MockService::new();
        let mut mock_esp32_initialize_service = crate::esp32::initialize_service::MockService::new();
        let mut mock_gnss_monitor_service = crate::gnss::monitor_service::MockService::new();

        let mut seq = mockall::Sequence::new();
        mock_esp32_initialize_service
            .expect_execute()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| ());
        mock_gnss_monitor_service
            .expect_execute()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| std::thread::spawn(|| {}));
        mock_core_event_loop_service
            .expect_execute()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| Ok(()));
        mock_esp32_halt_service
            .expect_execute()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| ());

        let module = TestModule::builder()
            .with_component_override::<dyn crate::esp32::halt_service::Interface>(Box::new(mock_esp32_halt_service))
            .with_component_override::<dyn crate::esp32::initialize_service::Interface>(Box::new(
                mock_esp32_initialize_service,
            ))
            .with_component_override::<dyn crate::core::event_loop_service::Interface>(Box::new(
                mock_core_event_loop_service,
            ))
            .with_component_override::<dyn crate::gnss::monitor_service::Interface>(Box::new(mock_gnss_monitor_service))
            .build();

        HasComponent::<dyn crate::core::main_service::Interface>::resolve(&module)
            .execute()
            .unwrap();
    }

    #[test]
    fn test_error_propagation_from_event_loop_service() {
        let mut mock_core_event_loop_service = crate::core::event_loop_service::MockService::new();
        let mut mock_esp32_halt_service = crate::esp32::halt_service::MockService::new();
        let mut mock_esp32_initialize_service = crate::esp32::initialize_service::MockService::new();
        let mut mock_gnss_monitor_service = crate::gnss::monitor_service::MockService::new();

        mock_esp32_initialize_service.expect_execute().times(1).returning(|| ());
        mock_gnss_monitor_service
            .expect_execute()
            .times(1)
            .returning(|_| std::thread::spawn(|| {}));
        mock_core_event_loop_service.expect_execute().times(1).returning(|_| {
            Err(crate::core::event_loop_service::Error::ReceiveError(
                std::sync::mpsc::RecvError,
            ))
        });
        mock_esp32_halt_service.expect_execute().times(1).returning(|| ());

        let module = TestModule::builder()
            .with_component_override::<dyn crate::esp32::halt_service::Interface>(Box::new(mock_esp32_halt_service))
            .with_component_override::<dyn crate::esp32::initialize_service::Interface>(Box::new(
                mock_esp32_initialize_service,
            ))
            .with_component_override::<dyn crate::core::event_loop_service::Interface>(Box::new(
                mock_core_event_loop_service,
            ))
            .with_component_override::<dyn crate::gnss::monitor_service::Interface>(Box::new(mock_gnss_monitor_service))
            .build();

        let result = HasComponent::<dyn crate::core::main_service::Interface>::resolve(&module).execute();
        assert_eq!(
            result,
            Err(crate::core::main_service::Error::EventLoopService(
                crate::core::event_loop_service::Error::ReceiveError(std::sync::mpsc::RecvError)
            ))
        );
    }
}
