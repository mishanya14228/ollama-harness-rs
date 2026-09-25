use crate::widgets::message_list::{ChatMessage, ChatRole};
use chrono::Local;

pub struct MessagesStorage {
    pub entries: Vec<ChatMessage>,
}

impl MessagesStorage {
    pub fn append_message(&mut self, role: ChatRole, content: String) {
        self.entries.push(ChatMessage {
            role,
            content,
            timestamp: Local::now(),
        });
    }
}

impl Default for MessagesStorage {
    fn default() -> Self {
        Self {
            entries: vec![ChatMessage {
                content: "Welcome to ollama-harness! Chat with your local Ollama models right from the terminal.

Type / to see available commands:
  /list-models       List all available models
  /create-assistant  Create a new assistant
  /select-assistant  Select an assistant to chat with
  /remove-assistant  Remove one or more assistants

Type @ to reference a file."
                    .to_string(),
                timestamp: Local::now(),
                role: ChatRole::App,
            }],
        }
    }
}
