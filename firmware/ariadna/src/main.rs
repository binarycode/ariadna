mod app_module;
mod event;
mod services;

use shaku::HasComponent;

fn main() {
    let module = app_module::AppModule::builder().build();
    let (_tx, rx) = std::sync::mpsc::channel();

    let event_loop_service: std::sync::Arc<dyn services::EventLoopServiceInterface> = module.resolve();

    event_loop_service.run(rx).unwrap();
}
