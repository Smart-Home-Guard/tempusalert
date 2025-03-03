use std::any::Any;
use std::sync::{Arc, Weak};

use aide::axum::ApiRouter;
use axum::async_trait;
use schemars::JsonSchema;
use serde::Serialize;

use crate::backend_core::features::fire_alert_feature::iot::IotFireFeature;
use crate::backend_core::features::{IotFeature, WebFeature};
use crate::backend_core::utils::non_primitive_cast;

mod routes;

#[derive(Serialize, JsonSchema)]
pub struct FireResponse {
    pub status: String,
    pub message: String,
}

#[derive(Clone)]
pub struct WebFireFeature {
    mongoc: mongodb::Client,
    iot_instance: Option<Weak<IotFireFeature>>,
    jwt_key: String,
}

#[async_trait]
impl WebFeature for WebFireFeature {
    fn create(
        mongoc: mongodb::Client,
        jwt_key: String,
    ) -> Option<Self> {
        Some(WebFireFeature {
            mongoc,
            iot_instance: None,
            jwt_key,
        })
    }

    fn name() -> String
    where
        Self: Sized,
    {
        "fire-alert".into()
    }

    fn get_module_name(&self) -> String {
        "fire-alert".into()
    }

    fn create_router(&mut self) -> ApiRouter {
        routes::create_router(self)
    }

    fn set_iot_feature_instance<I: IotFeature + 'static>(&mut self, iot_instance: Weak<I>)
    where
        Self: Sized,
    {
        self.iot_instance = Some(non_primitive_cast(iot_instance.clone()).unwrap());
    }
    
    fn get_iot_feature_instance(&self) -> Arc<dyn IotFeature + Send + Sync> {
        self.iot_instance.as_ref().unwrap().upgrade().unwrap()
    }

    async fn send_message_to_iot(&self, message: String) -> String { String::from("") }
    async fn respond_message_from_iot(&self, message: String) -> String { String::from("") }

    fn into_any(self: Arc<Self>) -> Arc<dyn Any> {
        self
    }
}
