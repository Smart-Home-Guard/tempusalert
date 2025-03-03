use aide::{
    axum::{routing::{get_with, post_with}, ApiRouter, IntoApiResponse},
    transform::TransformParameter,
};
use axum::{
    extract::Path,
    http::{HeaderMap, StatusCode},
};
use mongodb::{
    bson::{doc, to_bson, Document},
    Collection,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tempusalert_be::{backend_core::models::PushCredential, json::Json, push_notification::PUBLIC_KEY};

use crate::database_client::{init_database, MONGOC};

#[derive(Deserialize, JsonSchema)]
struct Params {
    email: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct PushCredentialBody {
    credential: PushCredential,
}

#[derive(Serialize, JsonSchema)]
struct PushCredentialResponse {
    message: String,
}

async fn register_push_credential_handler(
    headers: HeaderMap,
    Path(Params { email }): Path<Params>,
    Json(body): Json<PushCredentialBody>,
) -> impl IntoApiResponse {
    if headers.get("email").is_none()
        || headers
            .get("email")
            .is_some_and(|value| value != email.as_str())
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(PushCredentialResponse {
                message: String::from("Unauthorized"),
            }),
        );
    }
    let mongoc = MONGOC.get_or_init(init_database).await;
    let push_cred_coll: Collection<Document> = mongoc
        .default_database()
        .unwrap()
        .collection("push_credentials");
    if let Err(_) = push_cred_coll.insert_one(doc! { "endpoint": body.credential.endpoint, "key": to_bson(&body.credential.key).unwrap(), "email": body.credential.email }, None).await {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(PushCredentialResponse{ message: String::from("Failed to add subscription") }))
    } else {
        (StatusCode::OK, Json(PushCredentialResponse{ message: String::from("Successfully add user subscription") }))
    }
}

async fn get_public_key_handler() -> impl IntoApiResponse {
    let key = PUBLIC_KEY.clone();
    (StatusCode::OK, Json(key))
}

pub fn push_routes() -> ApiRouter {
    ApiRouter::new().api_route(
        "/:email",
        post_with(register_push_credential_handler, |op| {
            op.description("Add subscription for push notification")
                .tag("Push notification")
                .parameter("email", |op: TransformParameter<String>| {
                    op.description("The registered user's email")
                })
                .response::<200, Json<PushCredentialResponse>>()
                .response::<500, Json<PushCredentialResponse>>()
        })
    ).api_route(
    "/public-key",
        get_with(get_public_key_handler, |op|
            op.description("Get public key for client side subscription")
              .tag("Push notification")
              .response::<200, Json<String>>())
    )
}
