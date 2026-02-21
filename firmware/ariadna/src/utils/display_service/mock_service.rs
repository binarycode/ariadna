use shaku::Component;
use shaku::Module;
use shaku::ModuleBuildContext;

mockall::mock! {
    pub Service {}

    impl crate::utils::display_service::Interface for Service {
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
}

impl<M: Module> Component<M> for MockService {
    type Interface = dyn crate::utils::display_service::Interface;
    type Parameters = ();

    fn build(_: &mut ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        Box::new(Self::default())
    }
}
