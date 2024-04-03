use derive_more::Display;
use serde::{Deserialize, Serialize};

//TODO:: Make this into its own module

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
pub(crate) enum Channel {
	Default,
	HealthTest,
	FakeTest,
	Dev(DevChannel),
	CarSensorData,
	CameraImageRawData,
	LidarRangeImageRawData,
	LidarPointCloudRawData,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
pub enum DevChannel {
	DevChannel1,
}
