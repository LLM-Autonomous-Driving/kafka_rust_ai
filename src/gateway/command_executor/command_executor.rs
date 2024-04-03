use crate::application::services::car_service::CarService;
use crate::domain::common::error::Result;
use crate::domain::service_events::car_controller_event::{
	CarControllerEvent, CarControllerEventName,
};
use serde_json::json;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tokio::time::sleep;
use tokio::{fs, io};

pub struct CommandExecutor {
	pub data_file: String,
	pub car_service: CarService,
	pub delay_in_ms: u64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct DataValues {
	steering_angle: f32,
	speed: f32,
	brake: f32,
}

impl DataValues {
	pub fn new(steering_angle: f32, speed: f32, brake: f32) -> Self {
		Self {
			steering_angle,
			speed,
			brake,
		}
	}

	fn replace_third_comma(input_str: String) -> String {
		let mut comma_count = 0;
		let mut result = String::new();

		for c in input_str.chars() {
			if c == ',' {
				comma_count += 1;
				if comma_count == 3 {
					continue; // Skip adding this comma to the result
				}
			}
			result.push(c);
		}

		result
	}

	async fn parse_data_values(file_path: &str) -> Result<Vec<DataValues>> {
		let file_content = fs::read_to_string(file_path)
			.await
			.expect("Failed to read file");
		let mut data_values_list = Vec::new();

		let mut json_chunk = String::new();
		let mut inside_json = false;

		for c in file_content.chars() {
			if c == '{' {
				inside_json = true;
			}
			if inside_json {
				json_chunk.push(c);
			}
			if c == '}' {
				inside_json = false;
				json_chunk = Self::replace_third_comma(
					json_chunk.replace('\n', "").trim().to_string(),
				);
				data_values_list.push(json_chunk.clone());
				json_chunk.clear();
			}
		}

		Ok(data_values_list
			.iter()
			.map(|json_str| {
				serde_json::from_str(json_str).expect("Failed to parse json string")
			})
			.collect())
	}

	fn parse_input(input: &str) -> Option<DataValues> {
		let mut parts = input.split(',');

		// Ensure correct number of parts
		if parts.clone().count() != 2 {
			return None;
		}

		// Parse each part
		let steer = parts
			.next()?
			.trim()
			.split(':')
			.nth(1)?
			.trim()
			.parse()
			.ok()?;
		let speed = parts
			.next()?
			.trim()
			.split(':')
			.nth(1)?
			.trim()
			.parse()
			.ok()?;
		let brake = parts
			.next()?
			.trim()
			.split(':')
			.nth(1)?
			.trim()
			.parse()
			.ok()?;

		Some(DataValues::new(steer, speed, brake))
	}
}

impl CommandExecutor {
	pub fn new(
		data_file: String,
		car_service: CarService,
		delay_in_ms: u64,
	) -> Self {
		Self {
			data_file,
			car_service,
			delay_in_ms,
		}
	}

	pub async fn execute(&self, constant_speed: bool) {
		println!("Executing command..");

		let data_values = DataValues::parse_data_values(&self.data_file)
			.await
			.expect("Failed to parse data values..");
		for data in data_values {
			if constant_speed {
				self.car_service
					.handle_event(&CarControllerEvent::new(
						CarControllerEventName::SetSpeed,
						20.0,
					))
					.await
					.expect("Failed to handle the car event..");
			} else {
				self.car_service
					.handle_event(&CarControllerEvent::new(
						CarControllerEventName::SetSpeed,
						data.speed,
					))
					.await
					.expect("Failed to handle the car event..");
			}
			self.car_service
				.handle_event(&CarControllerEvent::new(
					CarControllerEventName::SetSteeringAngle,
					data.steering_angle,
				))
				.await
				.expect("Failed to handle the car event..");
			self.car_service
				.handle_event(&CarControllerEvent::new(
					CarControllerEventName::SetBrakeIntensity,
					data.brake,
				))
				.await
				.expect("Failed to handle the car event..");
			sleep(tokio::time::Duration::from_millis(self.delay_in_ms)).await;
		}

		// Read the data from the file line by line
		// let file = File::open(&self.data_file).expect("Failed to open file");
		// let mut reader = BufReader::new(file);
		// let mut line = String::new();
		// while reader.read_line(&mut line).unwrap() > 0 {
		// 	if let Some(data) = DataValues::parse_input(&line) {
		// 		println!("{:?}", data);
		// 		self.car_service
		// 			.handle_event(&CarControllerEvent::new(
		// 				CarControllerEventName::SetSpeed,
		// 				data.speed,
		// 			))
		// 			.await
		// 			.expect("Failed to handle the car event..");
		// 		self.car_service
		// 			.handle_event(&CarControllerEvent::new(
		// 				CarControllerEventName::SetSteeringAngle,
		// 				data.steer,
		// 			))
		// 			.await
		// 			.expect("Failed to handle the car event..");
		// 		self.car_service
		// 			.handle_event(&CarControllerEvent::new(
		// 				CarControllerEventName::SetBrakeIntensity,
		// 				data.brake,
		// 			))
		// 			.await
		// 			.expect("Failed to handle the car event..");
		// 	}
		//
		// 	line.clear();
		// 	sleep(tokio::time::Duration::from_millis(self.delay_in_ms)).await;
		// }
	}
}
