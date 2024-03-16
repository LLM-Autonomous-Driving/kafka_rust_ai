use crate::domain::common::error::Result;
use crate::domain::infrastructure::event_topic::EventTopics;

use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use rdkafka::ClientConfig;
use tokio::time::sleep;

pub struct KafkaAdmin {}

impl KafkaAdmin {
	async fn create_topic(hosts: &[String], topic: &EventTopics) -> Result<()> {
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
		Ok(())
	}

	pub async fn create_topics(
		hosts: &[String],
		topics: &[&EventTopics],
	) -> Result<()> {

		for topic in topics {
			let err_message = format!("Failed to create topic: {:?}", topic);

			KafkaAdmin::create_topic(hosts, topic)
				.await
				.expect(&err_message);

			println!("Created topic: {:?}", topic);
		}

		// Wait for system to finish creating the topics
		sleep(tokio::time::Duration::from_secs(3)).await;

		Ok(())
	}
}
