use crate::shared::autocomplete_state::{AutocompleteState, ReferenceType};
use crate::shared::chat_action::SubmitData;
use crate::shared::command::COMMANDS;
use crate::shared::debug_logger::DebugLogger;
use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Clone)]
pub struct TextInputState {
    pub input: String,
    pub input_mode: InputMode,
    pub character_index: usize,
    pub prefix: String,
    pub autocomplete_state: AutocompleteState,
    pub debug_logger: Arc<Mutex<DebugLogger>>,
    pub override_autocomplete_type: Option<ReferenceType>,
}

impl TextInputState {
    pub fn new(debug_logger: &Arc<Mutex<DebugLogger>>) -> Self {
        Self {
            prefix: String::from(" > | "),
            input: String::new(),
            input_mode: InputMode::Normal,
            character_index: 0,
            debug_logger: debug_logger.clone(),
            autocomplete_state: AutocompleteState::new(),
            override_autocomplete_type: None,
        }
    }

    pub fn reset(&mut self) {
        self.input.clear();
        self.character_index = 0;
        self.input_mode = InputMode::Normal;
    }
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
