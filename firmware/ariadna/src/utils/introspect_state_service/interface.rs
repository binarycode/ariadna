pub trait Interface: shaku::Interface {
    fn execute(&self, state: &crate::core::State);
}
