use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct AssistantConfig {
    pub name: String,
    pub model: String,
    pub system_prompt_path: String,
    pub use_rag: bool,
}
