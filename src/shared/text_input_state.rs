use crate::shared::autocomplete_state::AutocompleteState;
use crate::shared::chat_action::SubmitData;
use crate::shared::command::COMMANDS;
use crate::shared::debug_logger::DebugLogger;
use std::sync::{Arc, Mutex};

pub struct TextInputState {
    pub input: String,
    pub character_index: usize,
    pub prefix: String,
    pub autocomplete_state: AutocompleteState,
    pub debug_logger: Arc<Mutex<DebugLogger>>,
}

impl TextInputState {
    pub fn extract_submit_data(&self) -> SubmitData {
        let content = self.input.trim();
        let cmd = COMMANDS
            .iter()
            .find(|entry| entry.metadata().name == content.split('/').nth(1).unwrap_or(""));

        match cmd {
            None => SubmitData::Text(content.to_string()),
            Some(command) => SubmitData::Command(command.clone()),
        }
    }
}
