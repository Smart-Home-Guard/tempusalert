use aide::axum::ApiRouter;
use axum::async_trait;
use schemars::JsonSchema;
use serde::Serialize;
use tokio::sync::mpsc::{Receiver, Sender};

use super::notifications::{DeviceStatusIotNotification, DeviceStatusWebNotification};
use crate::backend_core::features::devices_status_feature::iot::IotDeviceStatusFeature;
use crate::backend_core::features::{IotFeature, WebFeature};
use crate::backend_core::utils::non_primitive_cast;

mod routes;

#[derive(Serialize, JsonSchema)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Clone)]
pub struct WebDeviceStatusFeature {
    mongoc: mongodb::Client,
    _iot_instance: Option<Box<IotDeviceStatusFeature>>,
    jwt_key: String,
}

#[async_trait]
impl WebFeature for WebDeviceStatusFeature {
    fn create<W: 'static, I: 'static>(
        mongoc: mongodb::Client,
        jwt_key: String,
    ) -> Option<Self> {
        Some(WebDeviceStatusFeature {
            mongoc,
            _iot_instance: None,
            jwt_key,
        })
    }

    fn name() -> String
    where
        Self: Sized,
    {
        "devices-status".into()
    }

    fn get_module_name(&self) -> String {
        "devices-status".into()
    }

    fn set_iot_feature_instance<I: IotFeature + 'static>(&mut self, iot_instance: I) {
        self._iot_instance = Some(Box::new(non_primitive_cast(iot_instance).unwrap()));
    }

    fn create_router(&mut self) -> ApiRouter {
        routes::create_router(self)
    }

    async fn process_next_iot_push_message(&mut self) {}
}
