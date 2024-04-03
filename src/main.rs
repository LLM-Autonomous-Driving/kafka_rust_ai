use crate::application::client::car_controller::CarController;
use crate::application::services::car_service::CarService;
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

use crate::gateway::command_creator::command_creator::CommandCreator;
use crate::gateway::command_executor::command_executor::CommandExecutor;
use std::ops::Deref;

mod application;
mod common;
mod core;
mod domain;
mod gateway;

#[derive(Debug)]
enum ProgramMode {
	CommandCreator,
	CommandExecutor,
	CreateThenExecute,
	Default,
}

#[tokio::main]
async fn main() {
	const PROGRAM_MODE: ProgramMode = ProgramMode::CommandExecutor;
	const CONSTANT_SPEED: bool = true;
	const DELAY_IN_MS: u64 = 3000;
	const DEFAULT_FILE: &str = "data/data_with_speed.txt";
	const DATA_WITH_SPEED_FILE: &str = "data/data_with_speed.txt";
	const DATA_WITHOUT_SPEED_FILE: &str = "data/data_without_speed.txt";
	const COMMANDS_FILE: &str = "data/commands.txt";
	const DEFAULT_AGENT_TOML_FILE: &str = "agent.toml";

	const DEFAULT_DIR: &str = "agent";
	const CAR_CONTROLLER_HOST: &str = "http://localhost:5300";

	let hosts = vec!["localhost:39093".to_owned()];

	let default_event: Event =
		Event::new(EventData::CameraImageRawData("test".to_string()));

	let topics: Vec<&EventTopics> = vec![
		&EventTopics::Default, // For Testing Purposes
		&EventTopics::CarSensorData,
		// &EventTopics::CameraImageRawData,
		// &EventTopics::LidarRangeImageRawData,
		// &EventTopics::LidarPointCloudRawData,
	];

	let agent =
		agent::Agent::init_from_dir(DEFAULT_DIR, true, DEFAULT_AGENT_TOML_FILE)
			.await
			.expect("Failed to initialise agent..");

	let car_service = CarService::new(CarController::new(CAR_CONTROLLER_HOST));

	// producer
	// 	.publish(&default_event, &"test".to_string())
	// 	.await
	// 	.expect("Failed to publish event..");

	match PROGRAM_MODE {
		ProgramMode::Default => {
			KafkaAdmin::create_topics(&hosts, &topics)
				.await
				.expect("Failed to create topics..");

			let producer = KafkaPublisher::new(&hosts);
			let consumer = KafkaConsumer::new(&hosts, &topics).await;

			let event_handler = EventHandlerLive::new(producer, agent, car_service);

			let kafka_gateway = KafkaGateway::new(consumer, event_handler);

			kafka_gateway.start(DEFAULT_FILE).await;
		}
		ProgramMode::CommandCreator => {
			let command_creator = CommandCreator::new(
				DATA_WITH_SPEED_FILE.parse().unwrap(),
				COMMANDS_FILE.parse().unwrap(),
				agent,
			);
			command_creator.create().await;
		}
		ProgramMode::CommandExecutor => {
			let command_executor = CommandExecutor::new(
				COMMANDS_FILE.parse().unwrap(),
				car_service,
				DELAY_IN_MS,
			);
			command_executor.execute(CONSTANT_SPEED).await;
			println!("Program Mode: {:?}", PROGRAM_MODE);
		}
		ProgramMode::CreateThenExecute => {
			let command_creator = CommandCreator::new(
				DATA_WITHOUT_SPEED_FILE.parse().unwrap(),
				COMMANDS_FILE.parse().unwrap(),
				agent,
			);
			command_creator.create().await;

			let command_executor = CommandExecutor::new(
				COMMANDS_FILE.parse().unwrap(),
				car_service,
				DELAY_IN_MS,
			);
			command_executor.execute(CONSTANT_SPEED).await;
		}
	}
}
