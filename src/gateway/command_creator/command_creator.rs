use crate::application::services::car_service::CarService;
use crate::core::agent::Agent;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

pub struct CommandCreator {
	pub data_file: String,
	pub write_to_file: String,
	pub agent: Agent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct DataWithSpeed {
	yellow_line_angle: f32,
	obstacle_angle: f32,
	obstacle_distance: f32,
	obstacle_steering: f32,
	steer: f32,
	speed: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct DataWithoutSpeed {
	yellow_line_angle: f32,
	obstacle_angle: f32,
	obstacle_distance: f32,
	obstacle_steering: f32,
	steer: f32,
}



impl DataWithSpeed {
	pub fn new(
		yellow_line_angle: f32,
		obstacle_angle: f32,
		obstacle_distance: f32,
		obstacle_steering: f32,
		steer: f32,
		speed: f32,
	) -> Self {
		Self {
			yellow_line_angle,
			obstacle_angle,
			obstacle_distance,
			obstacle_steering,
			steer,
			speed,
		}
	}

	fn parse_input(input: &str) -> Option<DataWithSpeed> {
		let mut parts = input.split(',');

		// Ensure correct number of parts
		if parts.clone().count() != 6 {
			return None;
		}

		// Parse each part
		let yellow_line_angle = parts
			.next()?
			.trim()
			.split(':')
			.nth(1)?
			.trim()
			.parse()
			.ok()?;
		let obstacle_angle = parts
			.next()?
			.trim()
			.split(':')
			.nth(1)?
			.trim()
			.parse()
			.ok()?;
		let obstacle_distance = parts
			.next()?
			.trim()
			.split(':')
			.nth(1)?
			.trim()
			.parse()
			.ok()?;
		let obstacle_steering = parts
			.next()?
			.trim()
			.split(':')
			.nth(1)?
			.trim()
			.parse()
			.ok()?;
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

		Some(DataWithSpeed::new(
			yellow_line_angle,
			obstacle_angle,
			obstacle_distance,
			obstacle_steering,
			steer,
			speed,
		))
	}

	pub fn to_json(&self) -> String {
		json!(self).to_string()
	}
}

impl DataWithoutSpeed {
	pub fn new(
		yellow_line_angle: f32,
		obstacle_angle: f32,
		obstacle_distance: f32,
		obstacle_steering: f32,
		steer: f32,
	) -> Self {
		Self {
			yellow_line_angle,
			obstacle_angle,
			obstacle_distance,
			obstacle_steering,
			steer,
		}
	}

	fn parse_input(input: &str) -> Option<DataWithoutSpeed> {
		let mut parts = input.split(',');

		// Ensure correct number of parts
		if parts.clone().count() != 5{
			return None;
		}

		// Parse each part
		let yellow_line_angle = parts
			.next()?
			.trim()
			.split(':')
			.nth(1)?
			.trim()
			.parse()
			.ok()?;
		let obstacle_angle = parts
			.next()?
			.trim()
			.split(':')
			.nth(1)?
			.trim()
			.parse()
			.ok()?;
		let obstacle_distance = parts
			.next()?
			.trim()
			.split(':')
			.nth(1)?
			.trim()
			.parse()
			.ok()?;
		let obstacle_steering = parts
			.next()?
			.trim()
			.split(':')
			.nth(1)?
			.trim()
			.parse()
			.ok()?;
		let steer = parts
			.next()?
			.trim()
			.split(':')
			.nth(1)?
			.trim()
			.parse()
			.ok()?;
		

		Some(DataWithoutSpeed::new(
			yellow_line_angle,
			obstacle_angle,
			obstacle_distance,
			obstacle_steering,
			steer,
		))
	}

	pub fn to_json(&self) -> String {
		json!(self).to_string()
	}
}

impl CommandCreator {
	pub fn new(data_file: String, write_to_file: String, agent: Agent) -> Self {
		Self {
			data_file,
			write_to_file,
			agent,
		}
	}

	pub async fn create(&self) {
		println!("Creating commands..");

		// Read the data from the file line by line
		let file = File::open(&self.data_file).expect("Failed to open file");
		let mut reader = BufReader::new(file);
		let mut line = String::new();
		while reader.read_line(&mut line).unwrap() > 0 {
			// Parse the line into a DataWithSpeed struct
			if let Some(data) = DataWithSpeed::parse_input(&line) {
				let response =
					self.agent.chat(data.to_json().as_str()).await.unwrap();
				let append = self
					.append_to_file(self.write_to_file.as_str(), response.clone())
					.await;
			} else if let Some(data) = DataWithoutSpeed::parse_input(&line) {
				let response =
					self.agent.chat(data.to_json().as_str()).await.unwrap();
				let append = self
					.append_to_file(self.write_to_file.as_str(), response.clone())
					.await;
			} else {
				println!("Failed to parse line: {}", line);
			}

			line.clear();
		}
	}

	async fn append_to_file(
		&self,
		file_path: &str,
		response: String,
	) -> crate::domain::common::error::Result<()> {
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
