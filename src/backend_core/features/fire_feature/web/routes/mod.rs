use std::sync::Arc;

use aide::axum::ApiRouter;
use tokio::sync::Mutex;

use super::WebFireFeature;

pub static mut MONGOC: Option<Arc<Mutex<mongodb::Client>>> = None;

// mod get_all_devices;
mod get_all_log_of_user;

pub fn create_router(web: &mut WebFireFeature) -> ApiRouter {
    unsafe {
        MONGOC = Some(Arc::new(Mutex::new(web.mongoc.clone())));
    }

    ApiRouter::new().nest("/", get_all_log_of_user::routes())
}
