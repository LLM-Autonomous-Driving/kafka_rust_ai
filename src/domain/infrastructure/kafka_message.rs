use crate::domain::infrastructure::channel::Channel;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaMessage {
	pub key: Uuid,
	pub payload: String,
	pub channel: Channel,
	pub timestamp: DateTime<Utc>,
}
