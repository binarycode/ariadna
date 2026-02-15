#[derive(Debug)]
pub enum Event {
    GnssFix { latitude: f64, longitude: f64 },
    Halt,
    IntrospectState,
}
