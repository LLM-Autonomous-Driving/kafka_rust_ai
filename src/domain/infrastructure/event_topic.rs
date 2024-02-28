use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
pub enum EventTopics {
	DefaultTopic,
	HealthTestTopic,
	ExternalTopic,
}

impl EventTopics {
	pub fn to_json(&self) -> String {
		json!(self).to_string()
	}

	pub fn event_from_json(topic: &str) -> EventTopics {
		println!("topic: {:?}", topic);
		serde_json::from_str(topic).unwrap()
	}
}
