use shaku::module;

module! {
    pub AppModule {
        components = [crate::services::EventLoopService],
        providers = []
    }
}
