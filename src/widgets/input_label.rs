use crate::InputMode;
use crate::widgets::input::TextInputState;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Modifier, StatefulWidget, Style, Stylize, Text, Widget};
use ratatui::widgets::Paragraph;

pub struct InputLabel<'a> {
    pub input_mode: &'a InputMode,
}

impl<'a> InputLabel<'a> {
    pub fn new(input_mode: &'a InputMode) -> InputLabel<'a> {
        InputLabel { input_mode }
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

impl<'a> StatefulWidget for InputLabel<'a> {
    type State = TextInputState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let token = state.reference_token.clone().unwrap_or_else(|| String::from(""));
        match token.chars().next() {
            Some('/') => {
                self.render_commands(format!("command: {}", token), area, buf);
            }
            Some('@') => {
                self.render_file_path(format!("file path: {}", token), area, buf);
            }
            _ => {
                self.render_default_text(area, buf);
            }
        }
    }
}
