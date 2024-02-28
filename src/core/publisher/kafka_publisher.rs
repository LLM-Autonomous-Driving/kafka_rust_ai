use crate::core::publisher::event_publisher::EventPublisher;
use crate::domain::common::error::Result;
use crate::domain::infrastructure::event::Event;

use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use std::time::Duration;

pub struct KafkaPublisher {
	producer: FutureProducer,
}

impl KafkaPublisher {
	pub fn new(hosts: &Vec<String>) -> Self {
		Self {
			producer: ClientConfig::new()
				.set("bootstrap.servers", hosts.join(","))
				.set("message.timeout.ms", "5000")
				.create()
				.expect("Producer creation error"),
		}
	}
}

impl EventPublisher for KafkaPublisher {
	async fn publish(&mut self, event: &Event, key: &String) -> Result<()> {
		self.producer
			.send(
				FutureRecord::to(&event.get_topic().to_string())
					.payload(&event.to_json())
					.key(&key),
				Duration::from_secs(5),
			)
			.await
			.expect("Failed to publish message");
		Ok(())
	}
}
