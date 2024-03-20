use crate::application::services::car_service::CarService;
use crate::application::client::car_controller::CarController;
use crate::core::agent;
use crate::core::kafka_admin::KafkaAdmin;
use crate::core::consumer::kafka_consumer::KafkaConsumer;
use crate::core::publisher::event_publisher::EventPublisher;
use crate::core::publisher::kafka_publisher::KafkaPublisher;
use crate::domain::infrastructure::event::Event;
use crate::domain::infrastructure::event_data::EventData;
use crate::domain::infrastructure::event_topic::EventTopics;
use crate::gateway::infrastructure::kafka_gateway::KafkaGateway;
use crate::gateway::infrastructure::live::event_handler_live::EventHandlerLive;

use std::ops::Deref;

mod application;
mod common;
mod core;
mod domain;
mod gateway;

#[tokio::main]
async fn main() {
	const DEFAULT_DIR: &str = "agent";
	const CAR_CONTROLLER_HOST: &str = "http://localhost:5300";

	let hosts = vec!["localhost:39093".to_owned()];

	let default_event: Event =
		Event::new(EventData::CameraImageRawData("test".to_string()));

	let topics: Vec<&EventTopics> = vec![
		&EventTopics::Default, // For Testing Purposes
		&EventTopics::CameraImageRawData,
		&EventTopics::LidarRangeImageRawData,
		&EventTopics::LidarPointCloudRawData,
	];

	KafkaAdmin::create_topics(&hosts, &topics)
		.await
		.expect("Failed to create topics..");

	let producer = KafkaPublisher::new(&hosts);
	let consumer = KafkaConsumer::new(&hosts, &topics).await;

	let agent = agent::Agent::init_from_dir(DEFAULT_DIR, false)
		.await
		.expect("Failed to initialise agent..");

	let car_service = CarService::new(
		CarController::new(CAR_CONTROLLER_HOST),
	);

	// producer
	// 	.publish(&default_event, &"test".to_string())
	// 	.await
	// 	.expect("Failed to publish event..");

	let event_handler = EventHandlerLive::new(producer, agent, car_service);

	let kafka_gateway = KafkaGateway::new(consumer, event_handler);

	kafka_gateway.start().await;
}
