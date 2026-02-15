use shaku::Component;
use shaku::Module;
use shaku::ModuleBuildContext;

mockall::mock! {
    pub Service {}
    impl crate::gnss::uart_service::Interface for Service {
        fn read_line(&self) -> Result<String, crate::gnss::uart_service::Error>;
    }
}

impl<M: Module> Component<M> for MockService {
    type Interface = dyn crate::gnss::uart_service::Interface;
    type Parameters = ();

    fn build(_: &mut ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        Box::new(Self::default())
    }
}
