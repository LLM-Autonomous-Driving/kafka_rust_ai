use crate::domain::infrastructure::event::Event;

use crate::domain::common::error::Result;

pub trait EventHandler {
	async fn handle_event(&self, event: &Event) -> Result<()>;
}
