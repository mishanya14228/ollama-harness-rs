use crate::shared::autocomplete_state::AutocompleteState;
use crate::shared::debug_logger::DebugLogger;
use std::sync::{Arc, Mutex};

pub struct TextInputState {
    pub input: String,
    pub character_index: usize,
    pub prefix: String,
    pub autocomplete_state: AutocompleteState,
    pub debug_logger: Arc<Mutex<DebugLogger>>,
}
