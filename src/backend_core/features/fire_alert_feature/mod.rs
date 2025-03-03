pub mod fixed_value;
mod iot;
pub mod models;
mod notifications;
mod web;

pub use iot::IotFireFeature as IotFeature;
pub use notifications::FireIotNotification as IotNotification;
pub use notifications::FireWebNotification as WebNotification;
pub use web::WebFireFeature as WebFeature;
pub static MUST_ON: bool = false;
