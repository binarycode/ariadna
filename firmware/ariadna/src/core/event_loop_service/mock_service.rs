use std::sync::mpsc::Receiver;

use shaku::Component;
use shaku::Module;
use shaku::ModuleBuildContext;

mockall::mock! {
    pub Service {}
    impl crate::core::event_loop_service::Interface for Service {
        fn execute(&self, rx: Receiver<crate::core::Event>) -> Result<(), crate::core::event_loop_service::Error>;
    }
}

impl<M: Module> Component<M> for MockService {
    type Interface = dyn crate::core::event_loop_service::Interface;
    type Parameters = ();

    fn build(_: &mut ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        Box::new(Self::default())
    }
}
