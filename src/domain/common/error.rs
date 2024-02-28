pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

// use derive_more::From;
//
// #[derive(Debug, From)]
// pub enum Error {
//     Io(std::io::Error),
//     Kafka(rdkafka::error::KafkaError),
//     SeaOrm(sea_orm::DbErr),
//     Sqlx(sqlx::Error),
//     Tracing(tracing::error::SpanTraceError),
// }
