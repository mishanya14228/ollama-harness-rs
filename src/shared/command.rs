#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CommandMetadata {
    pub name: &'static str,
    pub description: &'static str,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Command {
    ListModels(CommandMetadata),
    CreateAssistant(CommandMetadata),
    SelectAssistant(CommandMetadata),
}

impl Command {
    pub fn metadata(&self) -> &CommandMetadata {
        match self {
            Command::ListModels(meta) => meta,
            Command::CreateAssistant(meta) => meta,
            Command::SelectAssistant(meta) => meta,
        }
    }
}

pub const COMMANDS: &[Command] = &[
    Command::ListModels(CommandMetadata {
        name: "list-models",
        description: "List all available models",
    }),
    Command::CreateAssistant(CommandMetadata {
        name: "create-assistant",
        description: "Create a new assistant",
    }),
    Command::SelectAssistant(CommandMetadata {
        name: "select-assistant",
        description: "Select an assistant to chat with",
    }),
];
