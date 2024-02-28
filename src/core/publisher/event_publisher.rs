use crate::domain::common::error::Result;
use crate::domain::infrastructure::event::Event;

pub trait EventPublisher {
	async fn publish(&mut self, event: &Event, key: &String) -> Result<()>;
}
