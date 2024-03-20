use crate::application::client::car_controller::CarController;
use crate::domain::common::error::Result;
use crate::domain::service_events::car_controller_event::{
	CarControllerEvent, CarControllerEventName,
};

pub struct CarService {
	pub car_controller: CarController,
}

impl CarService {
	pub fn new(car_controller: CarController) -> Self {
		CarService { car_controller }
	}

	pub async fn handle_event(&self, event: &CarControllerEvent) -> Result<()> {
		println!("Handling event: {}", event.to_json());

        match event.get_name() {
			CarControllerEventName::SetSpeed => {
				self.car_controller.set_speed(event.get_value()).await
			}
			CarControllerEventName::SetSteeringAngle => {
				self.car_controller.set_steering_angle(event.get_value()).await
			}
			CarControllerEventName::SetBrakeIntensity => {
				self.car_controller.set_brake_intensity(event.get_value()).await
			}
			CarControllerEventName::SetIndicatorOff => {
				self.car_controller.set_indicator_off().await
			}
			CarControllerEventName::SetIndicatorLeft => {
				self.car_controller.set_indicator_left().await
			}
			CarControllerEventName::SetIndicatorRight => {
				self.car_controller.set_indicator_right().await
			}
			CarControllerEventName::Invalid => Err("Invalid event name".into()),
		}
	}
}
