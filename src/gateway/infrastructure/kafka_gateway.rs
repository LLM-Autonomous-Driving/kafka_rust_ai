use crate::core::consumer::event_consumer::EventConsumer;
use crate::core::consumer::kafka_consumer::KafkaConsumer;
use crate::core::publisher::kafka_publisher::KafkaPublisher;
use crate::gateway::infrastructure::event_handler::EventHandler;
use crate::gateway::infrastructure::live::event_handler_live::EventHandlerLive;
use tokio::time::sleep;

pub struct KafkaGateway<Consumer: EventConsumer, Handler: EventHandler> {
	consumer: Consumer,
	event_handler: Handler,
}

impl KafkaGateway<KafkaConsumer, EventHandlerLive<KafkaPublisher>> {
	pub fn new(
		consumer: KafkaConsumer,
		event_handler: EventHandlerLive<KafkaPublisher>,
	) -> Self {
		Self {
			consumer,
			event_handler,
		}
	}

	pub async fn start(&self, file_path: &str) {
		loop {
			let (event, message) = self
				.consumer
				.get_event()
				.await
				.expect("Issue retrieving message");

			self.event_handler
				.handle_event(&event, file_path)
				.await
				.expect("Issue handling event");

			self.consumer
				.commit_consumed(&event.get_topic().to_string(), &message)
				.await
				.expect("Issue committing consumed message");

			sleep(tokio::time::Duration::from_secs(2)).await;
		}
	}
}
