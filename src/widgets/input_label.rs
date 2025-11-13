use crate::InputMode;
use crate::widgets::input::TextInputState;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Modifier, Style, Stylize, Text, Widget};
use ratatui::widgets::Paragraph;

pub struct InputLabel<'a> {
    pub input_mode: &'a InputMode,
    pub input_state: &'a TextInputState,
}

impl<'a> InputLabel<'a> {
    pub fn new(input_mode: &'a InputMode, input_state: &'a TextInputState) -> InputLabel<'a> {
        InputLabel {
            input_mode,
            input_state,
        }
    }

    pub fn detect_token(&self) -> Option<&'a str> {
        if self.input_state.character_index > self.input_state.input.len() {
            return None;
        }

        // Find token start
        let mut start = self.input_state.character_index;
        while start > 0 {
            let ch = self.input_state.input.chars().nth(start - 1).unwrap();
            if ch.is_whitespace() {
                break;
            }
            start -= 1;
        }

        // Find token end
        let mut end = self.input_state.character_index;
        while end < self.input_state.input.len() {
            let ch = self.input_state.input.chars().nth(end).unwrap();
            if ch.is_whitespace() {
                break;
            }
            end += 1;
        }

        let token = &self.input_state.input[start..end];

        if token.starts_with('/') || token.starts_with('@') {
            Some(token)
        } else {
            None
        }
    }


    fn render_default_text(self, area: Rect, buf: &mut Buffer) {
        // rendering text block widget
        let (msg, style) = match self.input_mode {
            InputMode::Normal => (
                vec![
                    "Press ".into(),
                    "q".bold(),
                    " to exit, ".into(),
                    "e".bold(),
                    " to start editing.".bold(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    "Press ".into(),
                    "Esc".bold(),
                    " to stop editing, ".into(),
                    "Enter".bold(),
                    " to record the message".into(),
                ],
                Style::default(),
            ),
        };
        let text = Text::from(Line::from(msg)).patch_style(style);

        let help_message = Paragraph::new(text);
        help_message.render(area, buf);
    }

    fn render_commands(self, token: String, area: Rect, buf: &mut Buffer) {
        let text = Text::from(Line::from(token));
        let help_message = Paragraph::new(text);
        help_message.render(area, buf);
    }

    fn render_file_path(self, token: String, area: Rect, buf: &mut Buffer) {
        let text = Text::from(Line::from(token));
        let help_message = Paragraph::new(text);
        help_message.render(area, buf);
    }
}

impl<'a> Widget for InputLabel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {

        let token = self.detect_token().unwrap_or_else(|| "");
        match token.chars().next() {
            Some('/') => {
                self.render_commands(format!("command: {}", token), area, buf);
            },
            Some('@') => {
                self.render_file_path(format!("file path: {}", token), area, buf);
            },
            _ => {
                self.render_default_text(area, buf);
            }
        }
    }
}
