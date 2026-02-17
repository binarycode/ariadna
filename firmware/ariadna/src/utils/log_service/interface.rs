pub trait Interface: shaku::Interface {
    fn trace(&self, message: &str);
    fn debug(&self, message: &str);
    fn info(&self, message: &str);
    fn warn(&self, message: &str);
    fn error(&self, message: &str);
}
