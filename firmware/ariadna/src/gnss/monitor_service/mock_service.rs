use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

use shaku::Component;
use shaku::Module;
use shaku::ModuleBuildContext;

mockall::mock! {
    pub Service {}
    impl crate::gnss::monitor_service::Interface for Service {
        fn execute(&self, tx: Sender<crate::core::Event>) -> JoinHandle<()>;
    }
}

impl<M: Module> Component<M> for MockService {
    type Interface = dyn crate::gnss::monitor_service::Interface;
    type Parameters = ();

    fn build(_: &mut ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        Box::new(Self::default())
    }
}
