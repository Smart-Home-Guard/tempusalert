use mongodb::{bson::doc, Cursor};
use once_cell::sync::Lazy;
use web_push::{
    ContentEncoding, IsahcWebPushClient, SubscriptionInfo, VapidSignatureBuilder, WebPushClient,
    WebPushMessageBuilder,
};

use crate::{backend_core::models::{PushCredential, PushKey}, parse_env_var::parse_env_var};

static SECRET_KEY: Lazy<String> =
    Lazy::new(|| parse_env_var("PRIVATE_VAPID_KEY"));

pub static PUBLIC_KEY: Lazy<String> =
    Lazy::new(|| parse_env_var("PUBLIC_VAPID_KEY"));

pub async fn push_notification(email: String, message: String, mongoc: &mut mongodb::Client) -> Option<()> {
    let mut cred_cursor: Cursor<PushCredential> = mongoc
        .default_database()
        .unwrap()
        .collection("push_credentials")
        .find(doc! { "email": email }, None)
        .await
        .ok()?;

    while let Ok(true) = cred_cursor.advance().await {
        let cred = cred_cursor.deserialize_current().ok()?;
        let endpoint = cred.endpoint;
        let PushKey { p256dh, auth } = cred.key;

        let subscription_info = SubscriptionInfo::new(endpoint, p256dh, auth);

        let sig_builder =
            VapidSignatureBuilder::from_pem(SECRET_KEY.clone().into_bytes().as_slice(), &subscription_info)
                .ok()?
                .build()
                .ok()?;

        let mut builder = WebPushMessageBuilder::new(&subscription_info);
        let content = message.as_bytes();
        builder.set_payload(ContentEncoding::Aes128Gcm, content);
        builder.set_vapid_signature(sig_builder);

        let client = IsahcWebPushClient::new().ok()?;

        client.send(builder.build().ok()?).await.ok()?;
    }

    Some(())
}
