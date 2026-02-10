mod app_module;
mod event;
mod services;

use shaku::HasComponent;

fn main() {
    let module = app_module::AppModule::builder().build();
    let (_tx, rx) = std::sync::mpsc::channel();

    let esp32_service: std::sync::Arc<dyn services::Esp32ServiceInterface> = module.resolve();
    let event_loop_service: std::sync::Arc<dyn services::EventLoopServiceInterface> = module.resolve();

    esp32_service.init();
    event_loop_service.run(rx).unwrap();
    esp32_service.halt();
}
