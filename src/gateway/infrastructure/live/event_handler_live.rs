use crate::core::publisher::event_publisher::EventPublisher;
use crate::domain::common::error::Result;
use crate::domain::infrastructure::event::Event;
use crate::gateway::infrastructure::event_handler::EventHandler;

use crate::core::publisher::kafka_publisher::KafkaPublisher;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

pub struct EventHandlerLive<Publisher: EventPublisher> {
	publisher: Publisher,
}

impl EventHandler for EventHandlerLive<KafkaPublisher> {
	async fn handle_event(&mut self, event: &Event) -> Result<()> {
		println!("Handling event: {}", event.to_json());

		sleep(Duration::from_secs(2)).await;

		let key = Uuid::new_v4().to_string();

		self.publisher
			.publish(event, &key)
			.await
			.expect("Failed to publish event");
		Ok(())
	}
}

impl EventHandlerLive<KafkaPublisher> {
	pub fn new(publisher: KafkaPublisher) -> Self {
		EventHandlerLive { publisher }
	}
}
