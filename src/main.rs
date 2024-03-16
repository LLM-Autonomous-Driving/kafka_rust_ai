use crate::core::agent;
use crate::core::consumer::kafka_consumer::KafkaConsumer;
use crate::core::kafka_admin::KafkaAdmin;
use crate::core::publisher::event_publisher::EventPublisher;
use crate::core::publisher::kafka_publisher::KafkaPublisher;
use crate::domain::infrastructure::event::Event;
use crate::domain::infrastructure::event_data::EventData;
use crate::domain::infrastructure::event_topic::EventTopics;
use crate::gateway::infrastructure::kafka_gateway::KafkaGateway;
use crate::gateway::infrastructure::live::event_handler_live::EventHandlerLive;

use std::ops::Deref;
use tokio::time::sleep;

mod application;
mod common;
mod core;
mod domain;
mod gateway;

#[tokio::main]
async fn main() {
	const DEFAULT_DIR: &str = "agent";

	let hosts = vec!["localhost:29092".to_owned()];

	let default_event: Event = Event::new(EventData::Default);

	let topics: Vec<&EventTopics> = vec![
		&EventTopics::Default, // For Testing Purposes
		&EventTopics::CameraImageRawData,
		&EventTopics::LidarRangeImageRawData,
		&EventTopics::LidarPointCloudRawData,
	];

	KafkaAdmin::create_topics(&hosts, &topics)
		.await
		.expect("Failed to create topics..");

	let mut producer = KafkaPublisher::new(&hosts);
	let consumer = KafkaConsumer::new(&hosts, &topics).await;

	let agent = agent::Agent::init_from_dir(DEFAULT_DIR, false)
		.await
		.expect("Failed to initialise agent..");

	producer
		.publish(&default_event, &"test".to_string())
		.await
		.expect("Failed to publish event..");

	let event_handler = EventHandlerLive::new(producer, agent);

	let mut kafka_gateway = KafkaGateway::new(consumer, event_handler);

	kafka_gateway.start().await;
}
