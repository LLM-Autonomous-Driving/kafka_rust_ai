use crate::domain::common::error::Result;
use crate::domain::infrastructure::event::Event;
use rdkafka::message::OwnedMessage;

pub trait EventConsumer {
	async fn get_event(&self) -> Result<(Event, OwnedMessage)>;
	async fn commit_consumed(
		&self,
		topic: &str,
		message: &OwnedMessage,
	) -> Result<()>;
}
