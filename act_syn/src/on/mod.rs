pub use event::{Event, EVENTS};
pub use on::On;

pub mod event;
#[allow(clippy::module_inception)]
pub mod on;
