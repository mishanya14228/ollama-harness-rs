use crate::shared::command::Command;

pub enum WindowState {
    Default,
    CommandFlow(Command),
}
