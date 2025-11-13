use crate::InputMode;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::{StatefulWidget, Widget};
use ratatui::widgets::BorderType;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

pub enum InputAction {
    None,
    Submit(String),
    ExitEditingMode,
}

pub struct TextInput<'a> {
    pub input_mode: &'a InputMode,
}

impl<'a> TextInput<'a> {
    pub fn new(input_mode: &'a InputMode) -> TextInput<'a> {
        TextInput { input_mode }
    }

    pub fn handle_key_event(key_event: KeyEvent, state: &mut TextInputState) -> InputAction {
        match key_event.code {
            KeyCode::Enter => {
                let content = state.input.clone();
                state.input.clear();
                TextInput::reset_cursor(state);
                InputAction::Submit(content)
            }
            KeyCode::Char(to_insert) => {
                TextInput::enter_char(state, to_insert);
                InputAction::None
            }
            KeyCode::Backspace => {
                TextInput::delete_char(state, key_event.modifiers);
                InputAction::None
            }
            KeyCode::Left => {
                TextInput::move_cursor_left(state);
                InputAction::None
            }
            KeyCode::Right => {
                TextInput::move_cursor_right(state);
                InputAction::None
            }
            KeyCode::Esc => InputAction::ExitEditingMode,
            _ => InputAction::None,
        }
    }

    fn move_cursor_left(state: &mut TextInputState) {
        let cursor_moved_left = state.character_index.saturating_sub(1);
        state.character_index = TextInput::clamp_cursor(state, cursor_moved_left);
    }

    fn move_cursor_right(state: &mut TextInputState) {
        let cursor_moved_right = state.character_index.saturating_add(1);
        state.character_index = TextInput::clamp_cursor(state, cursor_moved_right);
    }

    fn enter_char(state: &mut TextInputState, new_char: char) {
        let index = TextInput::byte_index(state);
        state.input.insert(index, new_char);
        TextInput::move_cursor_right(state);
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(state: &TextInputState) -> usize {
        state
            .input
            .char_indices()
            .map(|(i, _)| i)
            .nth(state.character_index)
            .unwrap_or(state.input.len())
    }

    fn delete_char(state: &mut TextInputState, modifier: KeyModifiers) {
        let remove_whole_word = modifier == KeyModifiers::ALT;

        let is_not_cursor_leftmost = state.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = state.character_index;
            let mut from_left_to_current_index = current_index - 1;
            if remove_whole_word {
                let (lefthand, _) = state.input.split_at(current_index);
                let mut split: Vec<&str> = lefthand.split_whitespace().collect();
                split.pop();
                let joined_string = split.join(" ");
                from_left_to_current_index = joined_string.len() + 1;
            }

            // Getting all characters before the selected character.
            let before_char_to_delete = state.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = state.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            state.input = before_char_to_delete.chain(after_char_to_delete).collect();
            TextInput::move_cursor_left(state);
        }
    }

    fn clamp_cursor(state: &TextInputState, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, state.input.chars().count())
    }

    fn reset_cursor(state: &mut TextInputState) {
        state.character_index = 0;
    }
}

pub struct TextInputState {
    pub input: String,
    pub character_index: usize,
    pub prefix: String,
}

impl<'a> StatefulWidget for TextInput<'a> {
    type State = TextInputState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // rendering input
        let input_text = format!("{}{}", state.prefix, state.input);
        let input = Paragraph::new(input_text.as_str())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("Input"),
            );
        input.render(area, buf);
    }
}
