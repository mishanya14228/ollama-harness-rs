use crate::shared::any_error::AnyError;
use crate::shared::assistant_config::ActiveAssistant;
use crate::widgets::message_list::{ChatMessage, ChatRole};
use ollama_rs::Ollama;
use ollama_rs::generation::chat::ChatMessage as OllamaMessage;
use ollama_rs::generation::chat::request::ChatMessageRequest;

pub async fn send(
    ollama: &Ollama,
    assistant: &ActiveAssistant,
    history: &[ChatMessage],
) -> Result<String, AnyError> {
    let mut messages = vec![OllamaMessage::system(assistant.system_prompt.clone())];
    for message in history {
        match message.role {
            ChatRole::User => messages.push(OllamaMessage::user(message.content.clone())),
            ChatRole::Assistant => messages.push(OllamaMessage::assistant(message.content.clone())),
            ChatRole::App => {}
        }
    }

    let request = ChatMessageRequest::new(assistant.config.model.clone(), messages);
    let response = ollama.send_chat_messages(request).await?;

    Ok(strip_think_blocks(&response.message.content))
}

fn strip_think_blocks(content: &str) -> String {
    let mut result = content.to_string();
    while let Some(start) = result.find("<think>") {
        match result[start..].find("</think>") {
            Some(end) => result.replace_range(start..start + end + "</think>".len(), ""),
            None => result.truncate(start),
        }
    }
    result.trim().to_string()
}
