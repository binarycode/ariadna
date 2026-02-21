mod color;
mod error;
mod interface;
#[cfg(test)]
mod mock_service;
#[cfg(not(target_arch = "xtensa"))]
mod no_op_service;
#[cfg(target_arch = "xtensa")]
mod service;

pub use color::*;
pub use error::*;
pub use interface::*;
#[cfg(test)]
pub use mock_service::*;
#[cfg(not(target_arch = "xtensa"))]
pub use no_op_service::*;
#[cfg(target_arch = "xtensa")]
pub use service::*;
