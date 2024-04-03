use tokio::fs::File;
use crate::core::agent::Agent;

pub struct ImageLabeler {
	pub agent: Agent,
}

impl ImageLabeler {
	pub fn new(agent: Agent) -> Self {
		ImageLabeler { agent }
	}
	
}
