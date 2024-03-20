use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
pub enum EventData {
	HealthTest(HealthTestEventData),
	FakeTest,
	Default,
	CameraImageRawData(String),
	LidarRangeImageRawData(String),
	LidarPointCloudRawData(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
pub struct HealthTestEventData {
	service: String,
}

impl HealthTestEventData {
	pub fn new(service: String) -> HealthTestEventData {
		HealthTestEventData { service }
	}
}
