use rumqttc::{MqttOptions, Client, QoS};
use std::time::Duration;

pub struct IoTAgent { client: Option<Client> }

impl IoTAgent {
    pub fn new() -> Self { Self { client: None } }
    
    pub fn connect(&mut self, broker: &str, port: u16, id: &str) -> Result<(), String> {
        let mut opts = MqttOptions::new(id, broker, port);
        opts.set_keep_alive(Duration::from_secs(5));
        let (client, _) = Client::new(opts, 10);
        self.client = Some(client);
        Ok(())
    }
    
    pub fn publish(&self, topic: &str, msg: &str) -> Result<(), String> {
        self.client.as_ref()
            .ok_or("Chưa kết nối MQTT")?
            .publish(topic, QoS::AtLeastOnce, false, msg.as_bytes())
            .map_err(|e| format!("MQTT: {}", e))
    }
}
