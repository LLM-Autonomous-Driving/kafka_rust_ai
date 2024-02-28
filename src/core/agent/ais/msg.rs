use crate::domain::common::error::Result;

use async_openai::types::{CreateMessageRequest, MessageObject};

pub fn user_msg(content: impl Into<String>) -> CreateMessageRequest {
	CreateMessageRequest {
		role: "user".to_string(),
		content: content.into(),
		..Default::default()
	}
}

pub fn get_text_content(msg: MessageObject) -> Result<String> {
	let msg_content = msg.content.into_iter().next().ok_or("No content")?;

	let txt = match msg_content {
		async_openai::types::MessageContent::Text(txt) => txt.text.value,
		async_openai::types::MessageContent::ImageFile(_) => {
			return Err("Message Image not supported yet".into())
		}
	};

	Ok(txt)
}
