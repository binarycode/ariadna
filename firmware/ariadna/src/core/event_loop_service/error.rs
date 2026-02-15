#[derive(Debug)]
#[derive(PartialEq)]
#[derive(thiserror::Error)]
pub enum Error {
    #[error("Channel receive error: {0}")]
    ReceiveError(#[from] std::sync::mpsc::RecvError),
}
