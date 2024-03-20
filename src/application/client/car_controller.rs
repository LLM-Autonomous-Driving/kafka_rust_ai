use crate::domain::common::error::Result;

use reqwest::get;

pub struct CarController {
	url: &'static str,
}

pub enum CarIndicator {
	Left = 2,
	Right = 1,
	None = 0,
}

impl CarController {
	pub fn new(url: &'static str) -> CarController {
		CarController { url }
	}

	pub async fn stop(&self) -> Result<()> {
		get(format!("{}/stop", self.url)).await.unwrap();
		Ok(())
	}

	pub async fn start(&self) -> Result<()> {
		get(format!("{}/start", self.url)).await.unwrap();
		Ok(())
	}

	pub async fn get_steering_angle(&self) -> Result<f32> {
		let response = get(format!("{}/getSteeringAngle", self.url)).await.unwrap();
		let text = response.text().await.unwrap();
		Ok(text.parse().unwrap())
	}

	pub async fn set_steering_angle(&self, angle: f32) -> Result<()> {
		get(format!("{}/setSteeringAngle/{}", self.url, angle))
			.await
			.unwrap();
		Ok(())
	}

	pub async fn get_speed(&self) -> Result<f32> {
		let response = get(format!("{}/getSpeed", self.url)).await.unwrap();
		let text = response.text().await.unwrap();
		Ok(text.parse().unwrap())
	}

	pub async fn set_speed(&self, speed: f32) -> Result<()> {
		get(format!("{}/setSpeed/{}", self.url, speed))
			.await
			.unwrap();
		Ok(())
	}

	pub async fn get_brake_intensity(&self) -> Result<f32> {
		let response = get(format!("{}/getBrakeIntensity", self.url))
			.await
			.unwrap();
		let text = response.text().await.unwrap();
		Ok(text.parse().unwrap())
	}

	pub async fn set_brake_intensity(&self, intensity: f32) -> Result<()> {
		get(format!("{}/setBrakeIntensity/{}", self.url, intensity))
			.await
			.unwrap();
		Ok(())
	}

	pub async fn get_indicator(&self) -> Result<CarIndicator> {
		let response = get(format!("{}/getIndicator", self.url)).await.unwrap();
		let text = response.text().await.unwrap();
		Ok(match text.parse::<u32>() {
			Ok(2) => CarIndicator::Left,
			Ok(1) => CarIndicator::Right,
			_ => CarIndicator::None,
		})
	}

	pub async fn set_indicator_off(&self) -> Result<()> {
		get(format!("{}/setIndicatorOff", self.url)).await.unwrap();
		Ok(())
	}

	pub async fn set_indicator_left(&self) -> Result<()> {
		get(format!("{}/setIndicatorLeft", self.url)).await.unwrap();
		Ok(())
	}

	pub async fn set_indicator_right(&self) -> Result<()> {
		get(format!("{}/setIndicatorRight", self.url))
			.await
			.unwrap();
		Ok(())
	}
}
