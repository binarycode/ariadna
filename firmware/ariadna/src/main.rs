mod core;
mod esp32;
mod gnss;
mod utils;

#[cfg(target_arch = "xtensa")]
use esp_idf_hal::peripherals::Peripherals;
use shaku::HasComponent;

shaku::module! {
    pub Module {
        components = [
            core::event_loop_service::Service,
            core::main_service::Service,
            esp32::initialize_service::Service,
            esp32::halt_service::Service,
            gnss::monitor_service::Service,
            gnss::uart_service::Service,
            utils::display_service::Service,
            utils::introspect_state_service::Service,
            utils::log_service::Service,
        ],
        providers = []
    }
}

fn main() {
    #[cfg(not(target_arch = "xtensa"))]
    let builder = Module::builder();
    #[cfg(target_arch = "xtensa")]
    let mut builder = Module::builder();
    #[cfg(target_arch = "xtensa")]
    {
        let peripherals = Peripherals::take().unwrap();

        let parameters = gnss::uart_service::Service::build_parameters(
            peripherals.uart1,
            peripherals.pins.gpio43,
            peripherals.pins.gpio44,
        );
        builder = builder.with_component_parameters::<gnss::uart_service::Service>(parameters);

        let parameters = utils::display_service::Service::build_parameters(
            peripherals.spi2,
            peripherals.pins.gpio10,
            peripherals.pins.gpio11,
            peripherals.pins.gpio12,
            peripherals.pins.gpio13,
            peripherals.pins.gpio14,
        );
        builder = builder.with_component_parameters::<utils::display_service::Service>(parameters);
    }
    let module = builder.build();

    if let Err(e) = HasComponent::<dyn core::main_service::Interface>::resolve(&module).execute() {
        HasComponent::<dyn utils::log_service::Interface>::resolve(&module).error(&format!("Error in main: {e:?}"));
    }
}
