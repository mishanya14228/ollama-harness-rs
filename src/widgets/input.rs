use crate::InputMode;
use crate::shared::autocomplete_state::{AutocompleteState, ReferenceType};
use crate::shared::text_input_state::TextInputState;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::{StatefulWidget, Widget};
use ratatui::widgets::BorderType;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

#[derive(Debug, PartialEq, Eq)]
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
        let mut input_action = InputAction::None;

        match key_event.code {
            KeyCode::Enter => {
                let content = state.input.clone();
                state.input.clear();
                TextInput::reset_cursor(state);
                return InputAction::Submit(content);
            }
            KeyCode::Char(to_insert) => {
                TextInput::enter_char(state, to_insert);
                state.autocomplete_state.current_index = 0;
            }
            KeyCode::Backspace => {
                TextInput::delete_char(state, key_event.modifiers);
            }
            KeyCode::Left => {
                TextInput::move_cursor_left(state);
            }
            KeyCode::Right => {
                TextInput::move_cursor_right(state);
            }
            KeyCode::Esc => {
                input_action = InputAction::ExitEditingMode;
            }
            KeyCode::Down => state.autocomplete_state.increment_current_index(),
            KeyCode::Up => state.autocomplete_state.decrement_current_index(),
            KeyCode::Tab => {
                TextInput::autocomplete(state);
            }
            _ => {}
        }
        TextInput::detect_token(state);

        input_action
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

    fn autocomplete(state: &mut TextInputState) {
        let token_start_index = state.autocomplete_state.reference_token_index.clone();
        let token_length = state
            .autocomplete_state
            .reference_token
            .clone()
            .unwrap_or("".to_string())
            .len();
        let prefix = match state.autocomplete_state.reference_type {
            ReferenceType::Command => "/",
            ReferenceType::Filepath => "@",
            ReferenceType::None => "",
        };
        let to_insert = format!("{prefix}{}", state.autocomplete_state.get_selected_option());
        let before_char_to_delete = state.input.chars().take(token_start_index);
        let after_char_to_delete = state.input.chars().skip(token_length + token_start_index);
        state.input = before_char_to_delete
            .clone()
            .chain(after_char_to_delete)
            .collect();
        TextInput::move_cursor_left(state);

        state
            .input
            .insert_str(token_start_index, to_insert.as_str());

        state.autocomplete_state.set_current_index(0);
        state.autocomplete_state.set_options(vec![]);
        state.character_index = state.input.chars().count();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can contain multiple bytes, it's necessary to calculate
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

    pub fn detect_token(state: &mut TextInputState) -> Option<String> {
        if state.character_index > state.input.len() {
            return None;
        }

        // Find token start
        let mut start = state.character_index;
        while start > 0 {
            let ch = state.input.chars().nth(start - 1).unwrap();
            if ch.is_whitespace() {
                break;
            }
            start -= 1;
        }

        // Find token end
        let mut end = state.character_index;
        while end < state.input.len() {
            let ch = state.input.chars().nth(end).unwrap();
            if ch.is_whitespace() {
                break;
            }
            end += 1;
        }

        let token = &state.input[start..end];

        let ref_type = AutocompleteState::get_reference_type_from_token(token);
        match ref_type {
            ReferenceType::None => {
                state
                    .autocomplete_state
                    .set_reference_token(None, 0, ReferenceType::None);
                state.autocomplete_state.set_options(vec![]);
                None
            }
            ref_type => {
                state.autocomplete_state.set_reference_token(
                    Some(token.to_string()),
                    start,
                    ref_type,
                );
                Some(token.to_string())
            }
        }
    }
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
