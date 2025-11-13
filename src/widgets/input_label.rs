use crate::InputMode;
use crate::services::file_explorer::FileExplorer;
use crate::shared::text_input_state::TextInputState;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Modifier, StatefulWidget, Style, Stylize, Text, Widget};
use ratatui::widgets::Paragraph;
use std::env;

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

    fn render_commands(self, _state: &TextInputState, token: String, area: Rect, buf: &mut Buffer) {
        let text = Text::from(Line::from(token));
        let help_message = Paragraph::new(text);
        help_message.render(area, buf);
    }

    fn set_filepath_autocomplete(&self, state: &mut TextInputState, token: String) {
        let mut path = token.clone();
        path.remove(0);
        if let Some(stripped) = path.strip_prefix("~") {
            let home = env::var("HOME").unwrap();
            path = format!("{home}{stripped}");
        }
        let explorer = FileExplorer::new(state.debug_logger.clone());
        let options = explorer.list_dir(path.as_str());
        let lines: Vec<String> = options
            .into_iter()
            .filter(|entry| entry.contains(path.as_str()))
            .collect();
        state.autocomplete_state.set_options(lines.clone());
    }

    fn render_file_path(self, state: &mut TextInputState, area: Rect, buf: &mut Buffer) {
        let help_message = Paragraph::new(
            state
                .autocomplete_state
                .options
                .iter()
                .enumerate()
                .map(|(index, entry)| {
                    let style = if index == state.autocomplete_state.current_index {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::Cyan)
                    };
                    Line::from(entry.clone()).style(style)
                })
                .collect::<Vec<Line>>(),
        );
        help_message.render(area, buf);
    }
}

impl<'a> StatefulWidget for InputLabel<'a> {
    type State = TextInputState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let token = state
            .autocomplete_state
            .reference_token
            .clone()
            .unwrap_or_else(|| String::from(""));
        match token.chars().next() {
            Some('/') => {
                self.render_commands(state, token, area, buf);
            }
            Some('@') => {
                self.set_filepath_autocomplete(state, token);
                self.render_file_path(state, area, buf);
            }
            _ => {
                self.render_default_text(area, buf);
            }
        }
    }
}
