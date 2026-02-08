use std::sync::mpsc::Receiver;

use crate::event::Event;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(thiserror::Error)]
pub enum Error {
    #[error("channel receive error: {0}")]
    ReceiveError(#[from] std::sync::mpsc::RecvError),
}

pub trait EventLoopServiceInterface: shaku::Interface {
    fn run(&self, rx: Receiver<Event>) -> Result<(), Error>;
}

#[derive(shaku::Component)]
#[shaku(interface = EventLoopServiceInterface)]
pub struct EventLoopService;

impl EventLoopServiceInterface for EventLoopService {
    fn run(&self, rx: Receiver<Event>) -> Result<(), Error> {
        loop {
            match rx.recv() {
                Ok(Event::Halt) => {
                    break;
                }
                #[allow(unreachable_patterns)]
                Ok(_) => {}
                Err(e) => {
                    return Err(Error::ReceiveError(e));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_halt_terminates_loop() {
        let (tx, rx) = std::sync::mpsc::channel();

        tx.send(Event::Halt).unwrap();

        let result = EventLoopService.run(rx);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_receive_error_terminates_loop() {
        let (tx, rx) = std::sync::mpsc::channel::<Event>();

        drop(tx);

        let result = EventLoopService.run(rx);
        assert_eq!(result, Err(Error::ReceiveError(std::sync::mpsc::RecvError)));
    }
}
