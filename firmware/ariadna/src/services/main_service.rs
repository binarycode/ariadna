use std::sync::Arc;

use super::esp32_service;
use super::event_loop_service;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(thiserror::Error)]
pub enum Error {
    #[error("event loop service error: {0}")]
    EventLoopService(#[from] event_loop_service::Error),
}

pub trait MainServiceInterface: shaku::Interface {
    fn run(&self) -> Result<(), Error>;
}

#[derive(shaku::Component)]
#[shaku(interface = MainServiceInterface)]
pub struct MainService {
    #[shaku(inject)]
    esp32_service: Arc<dyn esp32_service::Esp32ServiceInterface>,
    #[shaku(inject)]
    event_loop_service: Arc<dyn event_loop_service::EventLoopServiceInterface>,
}

impl MainServiceInterface for MainService {
    fn run(&self) -> Result<(), Error> {
        let (_tx, rx) = std::sync::mpsc::channel();
        self.esp32_service.init();
        let result = self.event_loop_service.run(rx);
        self.esp32_service.halt();
        result.map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use shaku::HasComponent;

    use super::*;

    shaku::module! {
        TestModule {
            components = [MainService, esp32_service::mocks::MockEsp32Service, event_loop_service::mocks::MockEventLoopService],
            providers = []
        }
    }

    #[test]
    fn test_happy_case() {
        let mut mock_esp32_service = esp32_service::mocks::MockEsp32Service::new();
        let mut mock_event_loop_service = event_loop_service::mocks::MockEventLoopService::new();

        let mut seq = mockall::Sequence::new();
        mock_esp32_service
            .expect_init()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| ());
        mock_event_loop_service
            .expect_run()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|_| Ok(()));
        mock_esp32_service
            .expect_halt()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| ());

        let module = TestModule::builder()
            .with_component_override::<dyn esp32_service::Esp32ServiceInterface>(Box::new(mock_esp32_service))
            .with_component_override::<dyn event_loop_service::EventLoopServiceInterface>(Box::new(
                mock_event_loop_service,
            ))
            .build();

        HasComponent::<dyn MainServiceInterface>::resolve(&module)
            .run()
            .unwrap();
    }

    #[test]
    fn test_error_propagation_from_event_loop_service() {
        let mut mock_esp32_service = esp32_service::mocks::MockEsp32Service::new();
        let mut mock_event_loop_service = event_loop_service::mocks::MockEventLoopService::new();

        mock_esp32_service.expect_init().times(1).returning(|| ());
        mock_event_loop_service
            .expect_run()
            .times(1)
            .returning(|_| Err(event_loop_service::Error::ReceiveError(std::sync::mpsc::RecvError)));
        mock_esp32_service.expect_halt().times(1).returning(|| ());

        let module = TestModule::builder()
            .with_component_override::<dyn esp32_service::Esp32ServiceInterface>(Box::new(mock_esp32_service))
            .with_component_override::<dyn event_loop_service::EventLoopServiceInterface>(Box::new(
                mock_event_loop_service,
            ))
            .build();

        let result = HasComponent::<dyn MainServiceInterface>::resolve(&module).run();
        assert_eq!(
            result,
            Err(Error::EventLoopService(event_loop_service::Error::ReceiveError(
                std::sync::mpsc::RecvError
            )))
        );
    }
}
