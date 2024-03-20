use crate::domain::common::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarControllerEvent {
	name: CarControllerEventName,
	value: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CarControllerEventName {
	SetSpeed,
	SetSteeringAngle,
	SetBrakeIntensity,
	SetIndicatorOff,
	SetIndicatorLeft,
	SetIndicatorRight,
	Invalid,
}

impl CarControllerEventName {
	pub fn new(name: &str) -> Self {
		match name {
			"SetSpeed" => CarControllerEventName::SetSpeed,
			"SetSteeringAngle" => CarControllerEventName::SetSteeringAngle,
			"SetBrakeIntensity" => CarControllerEventName::SetBrakeIntensity,
			"SetIndicatorOff" => CarControllerEventName::SetIndicatorOff,
			"SetIndicatorLeft" => CarControllerEventName::SetIndicatorLeft,
			"SetIndicatorRight" => CarControllerEventName::SetIndicatorRight,
			_ => CarControllerEventName::Invalid,
		}
	}
}

impl CarControllerEvent {
	pub fn new(name: CarControllerEventName, value: f32) -> Self {
		CarControllerEvent { name, value }
	}
    
    pub fn get_name(&self) -> CarControllerEventName {
        self.name.clone()
    }
    
    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn to_json(&self) -> String {
        json!(self).to_string()
    }

	pub fn event_from_json(event: &str) -> Result<CarControllerEvent> {
		Ok(serde_json::from_str(event).unwrap())
	}
}
