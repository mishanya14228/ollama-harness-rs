use crate::shared::command::Command;

#[derive(Debug, PartialEq, Eq)]
pub enum InputAction {
    None,
    Submit(SubmitData),
    ExitEditingMode,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SubmitData {
    Text(String),
    Command(Command),
}
