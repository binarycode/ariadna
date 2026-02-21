pub trait Interface: shaku::Interface {
    fn clear(&self, color: crate::utils::display_service::Color) -> Result<(), crate::utils::display_service::Error>;
    fn draw_text(
        &self,
        text: &str,
        x: i32,
        y: i32,
        color: crate::utils::display_service::Color,
        background_color: Option<crate::utils::display_service::Color>,
    ) -> Result<(), crate::utils::display_service::Error>;
    fn size(&self) -> (u32, u32);
}
