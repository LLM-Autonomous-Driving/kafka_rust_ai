use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventData {
	HealthTest(HealthTestEventData),
	FakeTest,
	Default,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthTestEventData {
	service: String,
}

impl HealthTestEventData {
	pub fn new(service: String) -> HealthTestEventData {
		HealthTestEventData { service }
	}
}
