pub mod ais;
mod config;

use crate::common::utils::files::{
	ensure_dir, load_from_json, load_from_toml, read_to_string, save_to_json,
};
use crate::core::agent::ais::asst::{AsstId, ThreadId};
use crate::core::agent::ais::{asst, OaClient};
use crate::domain::common::error::Result;
use std::fs;

use crate::common::utils::cli::ico_check;
use crate::core::agent::config::Config;
use derive_more::{Deref, From};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const AGENT_TOML: &str = "agent.toml";

#[derive(Debug)]
pub struct Agent {
	dir: PathBuf,
	oac: OaClient,
	asst_id: AsstId,
	config: Config,
}

#[derive(Debug, From, Deref, Deserialize, Serialize)]
pub struct Conv {
	thread_id: ThreadId,
}

// Public Functions

impl Agent {
	pub fn name(&self) -> &str {
		&self.config.name
	}

	pub async fn init_from_dir(
		dir: impl AsRef<Path>,
		recreate_asst: bool,
	) -> Result<Self> {
		let dir = dir.as_ref();
		let config: Config = load_from_toml(dir.join(AGENT_TOML))?;

		let oac = crate::core::agent::ais::new_oa_client()?;
		let asst_id =
			asst::load_or_create(&oac, (&config).into(), recreate_asst).await?;

		let agent = Agent {
			dir: dir.to_path_buf(),
			oac,
			asst_id,
			config,
		};

		agent.upload_instructions().await?;

		Ok(agent)
	}

	pub async fn upload_instructions(&self) -> Result<bool> {
		let file = self.dir.join(&self.config.instructions_file);
		if file.exists() {
			let content = read_to_string(&file)?;
			asst::upload_instructions(&self.oac, &self.asst_id, content).await?;
			println!("{} Instructions uploaded", ico_check());
			Ok(true)
		} else {
			Ok(false)
		}
	}

	pub async fn load_or_create_conv(&self, recreate: bool) -> Result<Conv> {
		let conv_file = self.data_dir()?.join("conv.json");

		if recreate && conv_file.exists() {
			fs::remove_file(&conv_file)?;
		}

		let conv = if let Ok(conv) = load_from_json::<Conv>(&conv_file) {
			asst::get_thread(&self.oac, &conv.thread_id)
				.await
				.map_err(|_| format!("Could not find thread_id for: {:?}", conv))?;
			println!("{} Loaded conversation", ico_check());
			conv
		} else {
			let thread_id = asst::create_thread(&self.oac).await?;
			println!("{} Created conversation", ico_check());
			let conv = thread_id.into();
			save_to_json(&conv_file, &conv)?;
			conv
		};

		Ok(conv)
	}

	pub async fn chat(&self, conv: &Conv, msg: &str) -> Result<String> {
		let res =
			asst::run_thread_msg(&self.oac, &self.asst_id, &conv.thread_id, msg)
				.await?;
		Ok(res)
	}
}

//Private Functions

impl Agent {
	fn data_dir(&self) -> Result<PathBuf> {
		let data_dir = self.dir.join(".agent");
		ensure_dir(&data_dir)?;
		Ok(data_dir)
	}

	fn data_files_dir(&self) -> Result<PathBuf> {
		let dir = self.data_dir()?.join("files");
		ensure_dir(&dir)?;
		Ok(dir)
	}
}
