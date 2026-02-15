use shaku::Component;

#[derive(Component)]
#[shaku(interface = crate::utils::introspect_state_service::Interface)]
pub struct Service;

impl crate::utils::introspect_state_service::Interface for Service {
    fn execute(&self, _state: &crate::core::State) {}
}
