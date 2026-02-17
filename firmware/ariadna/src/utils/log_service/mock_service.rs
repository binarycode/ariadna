use shaku::Component;
use shaku::Module;
use shaku::ModuleBuildContext;

mockall::mock! {
    pub Service {}
    impl crate::utils::log_service::Interface for Service {
        fn trace(&self, message: &str);
        fn debug(&self, message: &str);
        fn info(&self, message: &str);
        fn warn(&self, message: &str);
        fn error(&self, message: &str);
    }
}

impl<M: Module> Component<M> for MockService {
    type Interface = dyn crate::utils::log_service::Interface;
    type Parameters = ();

    fn build(_: &mut ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        Box::new(Self::default())
    }
}
