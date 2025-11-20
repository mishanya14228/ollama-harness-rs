use crate::services::file_explorer::FileExplorer;
use crate::shared::command::COMMANDS;
use crate::shared::debug_logger::DebugLogger;
use crate::shared::text_input_state::TextInputState;
use crate::InputMode;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Modifier, StatefulWidget, Style, Stylize, Text, Widget};
use ratatui::widgets::Paragraph;
use std::env;

pub struct InputLabelWidget<'a> {
    pub input_mode: &'a InputMode,
}

impl<'a> InputLabelWidget<'a> {
    pub fn new(input_mode: &'a InputMode) -> InputLabelWidget<'a> {
        InputLabelWidget { input_mode }
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
                    " Use ".into(),
                    "@ or /".bold(),
                    " to reference files or use commands.".into(),
                ],
                Style::default(),
            ),
        };
        let text = Text::from(Line::from(msg)).patch_style(style);

        let help_message = Paragraph::new(text);
        help_message.render(area, buf);
    }

    fn set_commands_autocomplete(&self, state: &mut TextInputState, token: String) {
        let mut path = token.clone();
        DebugLogger::safe_log(&state.debug_logger, path.clone());
        path.remove(0);
        let max_name_length = COMMANDS
            .iter()
            .map(|cmd| cmd.metadata().name.len())
            .max()
            .unwrap_or(0);

        let lines: Vec<String> = COMMANDS
            .iter()
            .filter(|entry| entry.metadata().name.starts_with(&path))
            .map(|entry| {
                let meta = entry.metadata();
                format!(
                    "{:<width$} - {}",
                    meta.name,
                    meta.description,
                    width = max_name_length
                )
            })
            .collect();
        state.autocomplete_state.set_options(lines.clone());
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

    fn render_autocomplete(self, state: &mut TextInputState, area: Rect, buf: &mut Buffer) {
        let (total_options, start, end, current_index, options) =
            InputLabelWidget::prepare_autocomplete_options(state, area);

        if total_options == 0 {
            Paragraph::new("").render(area, buf);
            return;
        }

        let help_message = Paragraph::new(
            options[start..end]
                .iter()
                .enumerate()
                .map(|(offset, entry)| {
                    let absolute_index = start + offset;
                    let style = if absolute_index == current_index {
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

    fn prepare_autocomplete_options(
        state: &mut TextInputState,
        area: Rect,
    ) -> (usize, usize, usize, usize, Vec<String>) {
        let viewport_height = area.height.max(1) as usize;
        let total_options = state.autocomplete_state.options.len();

        let mut current_index = state.autocomplete_state.current_index;
        if current_index >= total_options {
            current_index = total_options - 1;
            state.autocomplete_state.set_current_index(current_index);
        }

        let max_start = total_options.saturating_sub(viewport_height);
        let start = current_index
            .saturating_sub(viewport_height.saturating_sub(1))
            .min(max_start);
        let end = (start + viewport_height).min(total_options);

        let options = &state.autocomplete_state.options;

        (total_options, start, end, current_index, options.clone())
    }
}

impl<'a> StatefulWidget for InputLabelWidget<'a> {
    type State = TextInputState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let token = state
            .autocomplete_state
            .reference_token
            .clone()
            .unwrap_or_else(|| String::from(""));
        match token.chars().next() {
            Some('/') => {
                if state.input.starts_with('/') {
                    self.set_commands_autocomplete(state, token);
                    self.render_autocomplete(state, area, buf);
                } else {
                    self.render_default_text(area, buf);
                }
            }
            Some('@') => {
                self.set_filepath_autocomplete(state, token);
                self.render_autocomplete(state, area, buf);
            }
            _ => {
                self.render_default_text(area, buf);
            }
        }
    }
}
