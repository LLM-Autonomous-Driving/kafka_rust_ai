use crate::common::utils::cli::{ico_check, ico_deleted_ok, ico_error};
use crate::core::agent::ais::msg::{get_text_content, user_msg};
use crate::core::agent::ais::OaClient;
use crate::domain::common::error::Result;

use async_openai::types::{
	AssistantObject, AssistantToolsRetrieval, CreateAssistantRequest,
	CreateRunRequest, CreateThreadRequest, ModifyAssistantRequest, RunStatus,
	ThreadObject,
};
use console::Term;
use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

const DEFAULT_QUERY: &[(&str, &str)] = &[("limit", "100")];
const POLL_INTERVAL: u64 = 500;

pub struct CreateConfig {
	pub name: String,
	pub model: String,
}

#[derive(Debug, From, Deref, Display, Serialize, Deserialize)]
pub struct AsstId(String);

#[derive(Debug, From, Deref, Display, Serialize, Deserialize)]
pub struct ThreadId(String);

#[derive(Debug, From, Deref, Display, Serialize, Deserialize)]
pub struct FileId(String);

async fn create(oac: &OaClient, config: CreateConfig) -> Result<AsstId> {
	let oa_assts = oac.assistants();

	let asst_obj = oa_assts
		.create(CreateAssistantRequest {
			model: config.model,
			name: Some(config.name),
			tools: Some(vec![AssistantToolsRetrieval::default().into()]),
			..Default::default()
		})
		.await?;

	println!("{} Asst created: {:?}", ico_check(), {
		asst_obj.name.as_ref().unwrap_or(&"unnamed".to_string())
	});

	Ok(asst_obj.id.into())
}

async fn first_by_name(
	oac: &OaClient,
	name: &String,
) -> Result<Option<AssistantObject>> {
	let oa_assts = oac.assistants();

	let assts = oa_assts.list(DEFAULT_QUERY).await?.data;

	let asst_obj = assts
		.into_iter()
		.find(|asst_obj| asst_obj.name.as_ref().map(|n| n == name).unwrap_or(false));

	let asst_clone = asst_obj.clone();

	match asst_clone {
		Some(asst_obj) => {
			println!("{} asst found: {:?}", ico_check(), { asst_obj.id })
		}
		None => println!("{} asst not found: {name:?}", ico_error()),
	}

	Ok(asst_obj)
}

async fn delete_by_name(oac: &OaClient, name: &String) -> Result<()> {
	match first_by_name(oac, name).await? {
		Some(asst_obj) => {
			let oa_assts = oac.assistants();

			//TODO: Delete all files

			oa_assts.delete(&asst_obj.id).await?;

			println!("{} Asst deleted: {:?}", ico_deleted_ok(), { asst_obj.id });
		}
		None => println!("{} Asst not found: {name:?}", ico_deleted_ok()),
	}

	Ok(())
}

pub async fn load_or_create(
	oac: &OaClient,
	config: CreateConfig,
	recreate: bool,
) -> Result<AsstId> {
	match first_by_name(oac, &config.name).await? {
		Some(asst_obj) => {
			if recreate {
				delete_by_name(oac, &config.name).await?;

				create(oac, config).await
			} else {
				println!("{} Asst loaded: {:?}", ico_check(), {
					asst_obj.name.as_ref().unwrap_or(&"unnamed".to_string())
				});
				Ok(asst_obj.id.into())
			}
		}
		None => create(oac, config).await,
	}
}

pub async fn upload_instructions(
	oac: &OaClient,
	asst_id: &AsstId,
	inst_content: String,
) -> Result<()> {
	let oa_assts = oac.assistants();
	let modif = ModifyAssistantRequest {
		instructions: Some(inst_content),
		..Default::default()
	};

	oa_assts.update(asst_id, modif).await?;
	Ok(())
}

pub async fn create_thread(oa_client: &OaClient) -> Result<ThreadId> {
	let oa_threads = oa_client.threads();

	let thread_obj = oa_threads
		.create(CreateThreadRequest {
			..Default::default()
		})
		.await?;

	Ok(thread_obj.id.into())
}

pub async fn get_thread(
	oa_client: &OaClient,
	thread_id: &ThreadId,
) -> Result<ThreadObject> {
	let oa_threads = oa_client.threads();

	let thread_obj = oa_threads.retrieve(thread_id).await?;

	Ok(thread_obj)
}

pub async fn run_thread_msg(
	oac: &OaClient,
	asst_id: &AsstId,
	thread_id: &ThreadId,
	msg: &str,
) -> Result<String> {
	let msg = user_msg(msg);

	// -- Attach message to thread

	let _message_obj = oac.threads().messages(thread_id).create(msg).await?;

	// -- Create a run for the thread
	let run_request = CreateRunRequest {
		assistant_id: asst_id.to_string(),
		..Default::default()
	};
	let run = oac.threads().runs(thread_id).create(run_request).await?;

	// -- Loop to get result
	let term = Term::stdout();
	loop {
		term.write_str(">")?;
		let run = oac.threads().runs(thread_id).retrieve(&run.id).await?;
		term.write_str("< ")?;
		match run.status {
			RunStatus::Completed => {
				term.write_str("\n")?;
				return get_first_thread_msg_content(oac, thread_id).await;
			}
			RunStatus::Queued | RunStatus::InProgress => (),
			other => {
				term.write_str("\n")?;
				return Err(format!("ERROR WHILE RUN: {:?}", other).into());
			}
		}
		sleep(Duration::from_millis(POLL_INTERVAL)).await;
	}
}

async fn get_first_thread_msg_content(
	oac: &OaClient,
	thread_id: &ThreadId,
) -> Result<String> {
	static QUERY: [(&str, &str); 1] = [("limit", "1")];

	let messages = oac.threads().messages(thread_id).list(&QUERY).await?;
	let msg = messages
		.data
		.into_iter()
		.next()
		.ok_or_else(|| "No message found".to_string())?;

	let text = get_text_content(msg)?;

	Ok(text)
}
