pub trait Esp32ServiceInterface: shaku::Interface {
    fn init(&self);
    fn halt(&self);
}

#[derive(shaku::Component)]
#[shaku(interface = Esp32ServiceInterface)]
pub struct Esp32Service;

impl Esp32ServiceInterface for Esp32Service {
    fn init(&self) {
        #[cfg(target_arch = "xtensa")]
        {
            esp_idf_svc::sys::link_patches();
            esp_idf_svc::log::EspLogger::initialize_default();
        }
    }

    fn halt(&self) {
        #[cfg(target_arch = "xtensa")]
        {
            loop {
                esp_idf_hal::delay::FreeRtos::delay_ms(u32::MAX);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_esp32_service_init_smoke() {
        Esp32Service.init();
    }

    #[test]
    fn test_esp32_service_halt_smoke() {
        Esp32Service.halt();
    }
}

#[cfg(test)]
pub mod mocks {
    use super::*;

    mockall::mock! {
        pub Esp32Service {}
        impl Esp32ServiceInterface for Esp32Service {
            fn init(&self);
            fn halt(&self);
        }
    }

    impl<M: shaku::Module> shaku::Component<M> for MockEsp32Service {
        type Interface = dyn Esp32ServiceInterface;
        type Parameters = ();

        fn build(_: &mut shaku::ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
            Box::new(Self::default())
        }
    }
}
