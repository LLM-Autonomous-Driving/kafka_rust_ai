use crate::core::consumer::kafka_consumer::KafkaConsumer;
use crate::core::kafka_admin::KafkaAdmin;
use crate::core::publisher::event_publisher::EventPublisher;
use crate::core::publisher::kafka_publisher::KafkaPublisher;
use crate::domain::infrastructure::event::Event;
use crate::domain::infrastructure::event_data::EventData;
use crate::gateway::infrastructure::kafka_gateway::KafkaGateway;
use crate::gateway::infrastructure::live::event_handler_live::EventHandlerLive;

mod application;
mod common;
mod core;
mod domain;
mod gateway;

#[tokio::main]
async fn main() {
	let hosts = vec!["localhost:29092".to_owned()];

	let event: Event = Event::new(EventData::Default);

	let topic = event.get_topic();

	KafkaAdmin::create_topic(&hosts, &topic)
		.await
		.expect("Failed to Initialise Topic..");

	let mut producer = KafkaPublisher::new(&hosts);
	let consumer = KafkaConsumer::new(&hosts, &[&topic]).await;

	producer
		.publish(&event, &"test".to_string())
		.await
		.expect("Failed to publish event..");

	let event_handler = EventHandlerLive::new(producer);

	let mut kafka_gateway = KafkaGateway::new(consumer, event_handler);

	kafka_gateway.start().await;
}
