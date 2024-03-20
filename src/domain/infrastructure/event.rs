use crate::domain::infrastructure::channel::Channel;
use crate::domain::infrastructure::event_data::EventData;
use crate::domain::infrastructure::event_topic::EventTopics;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
	channel: Channel,
	topic: EventTopics,
	data: EventData,
}

impl Event {
	pub fn new(event_data: EventData) -> Event {
		match event_data {
			EventData::HealthTest(data) => Event {
				channel: Channel::HealthTest,
				topic: EventTopics::HealthTest,
				data: EventData::HealthTest(data),
			},
			EventData::FakeTest => Event {
				channel: Channel::FakeTest,
				topic: EventTopics::External,
				data: EventData::FakeTest,
			},
			EventData::Default => Event {
				channel: Channel::Default,
				topic: EventTopics::Default,
				data: EventData::Default,
			},
			EventData::CameraImageRawData(data) => Event {
				channel: Channel::CameraImageRawData,
				topic: EventTopics::CameraImageRawData,
				data: EventData::CameraImageRawData(data),
			},
			EventData::LidarRangeImageRawData(data) => Event {
				channel: Channel::LidarRangeImageRawData,
				topic: EventTopics::LidarRangeImageRawData,
				data: EventData::LidarRangeImageRawData(data),
			},
			EventData::LidarPointCloudRawData(data) => Event {
				channel: Channel::LidarPointCloudRawData,
				topic: EventTopics::LidarPointCloudRawData,
				data: EventData::LidarPointCloudRawData(data),
			},
		}
	}

	pub fn get_channel(&self) -> Channel {
		self.channel.clone()
	}

	pub fn get_topic(&self) -> EventTopics {
		self.topic.clone()
	}

	pub fn get_data(&self) -> EventData {
		self.data.clone()
	}

	pub fn to_json(&self) -> String {
		json!(self).to_string()
	}

	pub fn event_from_json(event: &str) -> Event {
		serde_json::from_str(event).unwrap()
	}
}
