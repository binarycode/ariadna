use shaku::Component;
use shaku::Module;
use shaku::ModuleBuildContext;

mockall::mock! {
    pub Service {}
    impl crate::utils::introspect_state_service::Interface for Service {
        fn execute(&self, state: &crate::core::State);
    }
}

impl<M: Module> Component<M> for MockService {
    type Interface = dyn crate::utils::introspect_state_service::Interface;
    type Parameters = ();

    fn build(_: &mut ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        Box::new(Self::default())
    }
}
