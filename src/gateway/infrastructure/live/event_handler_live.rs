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

use crate::domain::infrastructure::channel::Channel;
use crate::domain::infrastructure::event_data::EventData;
use std::time::Duration;
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;
use uuid::Uuid;

pub struct EventHandlerLive<Publisher: EventPublisher> {
	publisher: Publisher,
	agent: Agent,
	car_service: CarService,
}

impl EventHandler for EventHandlerLive<KafkaPublisher> {
	async fn handle_event(&self, event: &Event, file_path: &str) -> Result<()> {
		println!("Handling event: {}", event.to_json());

		sleep(Duration::from_secs(2)).await;

		let key = Uuid::new_v4().to_string();

		match event.get_data() {
			EventData::CarSensorData(data) => {
				println!("Car sensor data: {:?}", data);

				let response = self.agent.chat(data.as_str()).await;
				let copy = response.unwrap();
				let append = self.append_to_file(file_path, copy.clone()).await;
				println!("Response: {:?}", copy.clone());
			}
			_ => {
				event.get_data();
			}
		}

		let car_controller_event =
			CarControllerEvent::new(CarControllerEventName::SetSpeed, 5.0);

		// println!("Response: {:?}", response);

		// self.car_service
		// 	.handle_event(&car_controller_event)
		// 	.await
		// 	.expect("Failed to handle the car event..");

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

	async fn append_to_file(&self, file_path: &str, response: String) -> Result<()> {
		// Result can be a file or an error
		let mut file = OpenOptions::new()
			.write(true)
			.append(true)
			.create(true)
			.open(file_path)
			.await
			.expect("Failed to open file");

		file.write_all(response.as_bytes())
			.await
			.expect("Failed to write to file");

		Ok(())
	}
}
