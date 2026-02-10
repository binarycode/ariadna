#[derive(Clone)]
#[derive(Debug)]
#[cfg_attr(not(test), expect(dead_code))]
pub enum Event {
    Halt,
}
