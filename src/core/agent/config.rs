use crate::core::agent::ais::asst;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
	pub name: String,
	pub model: String,
	pub instructions_file: String,
	pub file_bundles: Vec<FileBundle>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FileBundle {
	pub bundle_name: String,
	pub src_dir: String,
	pub dst_ext: String,
	pub src_globs: Vec<String>,
}

impl From<&Config> for asst::CreateConfig {
	fn from(config: &Config) -> Self {
		asst::CreateConfig {
			name: config.name.clone(),
			model: config.model.clone(),
		}
	}
}
