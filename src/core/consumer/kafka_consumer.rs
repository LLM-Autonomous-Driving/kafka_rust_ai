use crate::core::consumer::event_consumer::EventConsumer;
use crate::domain::common::error::Result;
use crate::domain::infrastructure::event::Event;
use crate::domain::infrastructure::event_topic::EventTopics;

use rdkafka::consumer::{CommitMode, Consumer as RDConsumer, StreamConsumer};
use rdkafka::message::OwnedMessage;
use rdkafka::{ClientConfig, Message as RDMessage, Offset, TopicPartitionList};
use std::str;
use uuid::Uuid;

pub struct KafkaConsumer {
	consumer: StreamConsumer,
}

impl EventConsumer for KafkaConsumer {
	async fn get_event(&self) -> Result<(Event, OwnedMessage)> {
		let message = self
			.consumer
			.recv()
			.await
			.expect("Failed to receive message");

		let owned_message = message.detach();
		let event = str::from_utf8(owned_message.payload().unwrap()).unwrap();

		Ok((Event::event_from_json(event), owned_message))
	}

	async fn commit_consumed(
		&self,
		topic: &str,
		message: &OwnedMessage,
	) -> Result<()> {
		let mut topic_partitions = TopicPartitionList::new();
		topic_partitions.add_partition_offset(
			topic,
			0,
			Offset::Offset(message.offset()),
		)?;
		self.consumer.commit(&topic_partitions, CommitMode::Async)?;
		Ok(())
	}
}

impl KafkaConsumer {
	pub async fn new(hosts: &Vec<String>, topics: &[&EventTopics]) -> Self {
		let consumer: StreamConsumer = ClientConfig::new()
			.set("bootstrap.servers", &hosts.join(","))
			.set("enable.partition.eof", "false")
			.set("group.id", format!("consumer-{}", Uuid::new_v4()))
			.set("auto.offset.reset", "earliest")
			.set("enable.auto.commit", "false")
			.create()
			.expect("Failed to create consumer");

		consumer
			.subscribe(
				&topics
					.iter()
					.map(|t| t.to_string())
					.collect::<Vec<String>>()
					.iter()
					.map(AsRef::as_ref)
					.collect::<Vec<&str>>(),
			)
			.expect("Failed to subscribe to topic");

		KafkaConsumer { consumer }
	}

	// pub fn convert_message_to_event(&self, message: &OwnedMessage) -> Result<Event> {
	// 	let event = str::from_utf8(message.payload().unwrap()).unwrap();
	// 	Ok(Event::event_from_json(event))
	// }

	// pub async fn get_event(&self) -> Result<(EventTopics, OwnedMessage)> {
	// 	let message = self
	// 		.consumer
	// 		.recv()
	// 		.await
	// 		.expect("Failed to receive message");
	//
	// 	let owned_message = message.detach();
	// 	let event = str::from_utf8(owned_message.payload().unwrap()).unwrap();
	//
	// 	Ok((EventTopics::event_from_json(event), owned_message))
	// }
	//
	//
	//
	// pub async fn commit_consumed(
	// 	&mut self,
	// 	topic: &str,
	// 	message: &OwnedMessage,
	// ) -> Result<()> {
	// 	let mut topic_partitions = TopicPartitionList::new();
	// 	topic_partitions.add_partition_offset(
	// 		topic,
	// 		0,
	// 		Offset::Offset(message.offset()),
	// 	)?;
	// 	self.consumer.commit(&topic_partitions, CommitMode::Async)?;
	// 	Ok(())
	// }
	//
	// fn subscribe(&mut self, topic: &EventTopics) -> Result<()> {
	// 	self.consumer
	// 		.subscribe(&[&topic.to_json()])
	// 		.expect("Failed to subscribe to topic");
	// 	Ok(())
	// }
}
