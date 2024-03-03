use crate::common::utils::cli::{ico_error, ico_res, prompt, text_res};
use crate::core::agent::Agent;
use crate::core::publisher::event_publisher::EventPublisher;
use crate::core::publisher::kafka_publisher::KafkaPublisher;
use crate::domain::common::error::Result;
use crate::domain::infrastructure::event::Event;
use crate::gateway::infrastructure::event_handler::EventHandler;

use std::time::Duration;
use textwrap::wrap;
use tokio::time::sleep;
use uuid::Uuid;

pub struct EventHandlerLive<Publisher: EventPublisher> {
	publisher: Publisher,
	agent: Agent,
}

impl EventHandler for EventHandlerLive<KafkaPublisher> {
	async fn handle_event(&mut self, event: &Event) -> Result<()> {
		println!("Handling event: {}", event.to_json());

		sleep(Duration::from_secs(2)).await;

		let key = Uuid::new_v4().to_string();
		let msg = prompt("Enter message: ").expect("Failed to get message");

		let res = self.agent.chat( &msg).await?;
		let res = wrap(&res, 80).join("\n");
		println!("{} {}", ico_res(), text_res(res));

		self.publisher
			.publish(event, &key)
			.await
			.expect("Failed to publish event");
		Ok(())
	}
}

impl EventHandlerLive<KafkaPublisher> {
	pub fn new(publisher: KafkaPublisher, agent: Agent) -> Self {
		EventHandlerLive { publisher, agent }
	}
}
