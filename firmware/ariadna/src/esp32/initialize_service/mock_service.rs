use shaku::Component;
use shaku::Module;
use shaku::ModuleBuildContext;

mockall::mock! {
    pub Service {}
    impl crate::esp32::initialize_service::Interface for Service {
        fn execute(&self);
    }
}

impl<M: Module> Component<M> for MockService {
    type Interface = dyn crate::esp32::initialize_service::Interface;
    type Parameters = ();

    fn build(_: &mut ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        Box::new(Self::default())
    }
}
