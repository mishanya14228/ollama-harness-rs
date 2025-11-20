use crate::shared::command::Command;

#[derive(Clone)]
pub enum WindowState {
    Default,
    CommandFlow(Command),
}
