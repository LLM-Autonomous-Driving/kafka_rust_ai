pub mod asst;
mod msg;

use crate::domain::common::error::Result;
use async_openai::config::OpenAIConfig;
use async_openai::Client;

const OPENAI_API_KEY: &str = "OPENAI_API_KEY";

pub type OaClient = Client<OpenAIConfig>;

pub fn new_oa_client() -> Result<OaClient> {
	if std::env::var(OPENAI_API_KEY).is_ok() {
		Ok(Client::new())
	} else {
		println!("{} not set", OPENAI_API_KEY);
		Err("OPENAI_API_KEY not set".into())
	}
}
