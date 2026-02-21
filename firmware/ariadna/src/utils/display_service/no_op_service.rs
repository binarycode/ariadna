#[derive(shaku::Component)]
#[shaku(interface = crate::utils::display_service::Interface)]
pub struct Service;

impl crate::utils::display_service::Interface for Service {
    fn clear(&self, _color: crate::utils::display_service::Color) -> Result<(), crate::utils::display_service::Error> {
        Ok(())
    }

    fn draw_text(
        &self,
        _text: &str,
        _x: i32,
        _y: i32,
        _color: crate::utils::display_service::Color,
        _background_color: Option<crate::utils::display_service::Color>,
    ) -> Result<(), crate::utils::display_service::Error> {
        Ok(())
    }

    fn size(&self) -> (u32, u32) {
        (240, 320)
    }
}
