use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AssistantConfig {
    pub name: String,
    pub model: String,
    pub system_prompt_path: String,
    pub use_rag: bool,
}

pub struct ActiveAssistant {
    pub config: AssistantConfig,
    pub system_prompt: String,
}
