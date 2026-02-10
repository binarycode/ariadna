mod app_module;
mod event;
mod services;

use shaku::HasComponent;

fn main() {
    let module = app_module::AppModule::builder().build();

    if let Err(e) = HasComponent::<dyn services::MainServiceInterface>::resolve(&module).run() {
        log::error!("Application error: {}", e);
    }
}
