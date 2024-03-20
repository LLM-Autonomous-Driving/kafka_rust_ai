use crate::application::services::car_service::CarService;
use crate::core::agent::Agent;
use crate::core::publisher::event_publisher::EventPublisher;
use crate::core::publisher::kafka_publisher::KafkaPublisher;
use crate::domain::common::error::Result;
use crate::domain::infrastructure::event::Event;
use crate::domain::service_events::car_controller_event::{
	CarControllerEvent, CarControllerEventName,
};
use crate::gateway::infrastructure::event_handler::EventHandler;

use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

pub struct EventHandlerLive<Publisher: EventPublisher> {
	publisher: Publisher,
	agent: Agent,
	car_service: CarService,
}

impl EventHandler for EventHandlerLive<KafkaPublisher> {
	async fn handle_event(&self, event: &Event) -> Result<()> {
		println!("Handling event: {}", event.to_json());

		sleep(Duration::from_secs(2)).await;

		let key = Uuid::new_v4().to_string();

		let car_controller_event =
			CarControllerEvent::new(CarControllerEventName::SetSpeed, 50.0);

		self.car_service
			.handle_event(&car_controller_event)
			.await
			.expect("Failed to handle the car event..");

		Ok(())
	}
}

impl EventHandlerLive<KafkaPublisher> {
	pub fn new(
		publisher: KafkaPublisher,
		agent: Agent,
		car_service: CarService,
	) -> Self {
		EventHandlerLive {
			publisher,
			agent,
			car_service,
		}
	}
}
