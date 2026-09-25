use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct AssistantConfig {
    pub name: String,
    pub model: String,
    pub system_prompt_path: String,
    pub use_rag: bool,
}

#[derive(Clone)]
pub struct StoredAssistant {
    pub path: PathBuf,
    pub config: AssistantConfig,
}

pub struct ActiveAssistant {
    pub path: PathBuf,
    pub config: AssistantConfig,
    pub system_prompt: String,
}
