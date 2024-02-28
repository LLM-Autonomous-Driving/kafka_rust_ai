use serde::{Deserialize, Serialize};

//TODO:: Make this into its own module

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum Channel {
	Default,
	HealthTest,
	FakeTest,
	Dev(DevChannel),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DevChannel {
	DevChannel1,
}
