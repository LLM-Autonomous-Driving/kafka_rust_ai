use crate::domain::infrastructure::event::Event;

use crate::domain::common::error::Result;

pub trait EventHandler {
	async fn handle_event(&mut self, event: &Event) -> Result<()>;
}
