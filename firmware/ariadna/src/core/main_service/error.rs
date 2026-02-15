#[derive(Debug)]
#[derive(PartialEq)]
#[derive(thiserror::Error)]
pub enum Error {
    #[error("Event loop service error: {0}")]
    EventLoopService(#[from] crate::core::event_loop_service::Error),
}
