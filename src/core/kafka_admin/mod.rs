use crate::domain::common::error::Result;
use crate::domain::infrastructure::event_topic::EventTopics;

use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use rdkafka::ClientConfig;
use tokio::time::sleep;

pub struct KafkaAdmin {}

impl KafkaAdmin {
	pub async fn create_topic(hosts: &[String], topic: &EventTopics) -> Result<()> {
		let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
			.set("bootstrap.servers", &hosts.join(","))
			.create()
			.expect("Failed to create client");
		admin_client
			.create_topics(
				&[NewTopic {
					name: &topic.to_string(),
					num_partitions: 1,
					replication: TopicReplication::Fixed(1),
					config: vec![],
				}],
				&AdminOptions::default(),
			)
			.await
			.expect("Failed to create topic");

		sleep(tokio::time::Duration::from_secs(3)).await;
		Ok(())
	}
}
