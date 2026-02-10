use shaku::module;

module! {
    pub AppModule {
        components = [crate::services::EventLoopService, crate::services::Esp32Service, crate::services::MainService],
        providers = []
    }
}
