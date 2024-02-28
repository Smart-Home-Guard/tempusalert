use crate::Feature;
use async_trait::async_trait;
use rumqttc::Client;

pub struct FeatureExample {}

impl FeatureExample {}

#[async_trait]
impl Feature for FeatureExample {
    fn name() -> String {
        "Feature Example".into()
    }

    async fn init(&mut self, rumqttc: &mut Client) {}

    async fn process_iot_message(&mut self, message: String) {}

    async fn process_push_notification(&mut self, message: String) {}

    async fn send_command(&mut self, command: String) {}
}
