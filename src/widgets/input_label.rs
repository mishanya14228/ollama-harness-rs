use crate::InputMode;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Modifier, Style, Stylize, Text, Widget};
use ratatui::widgets::Paragraph;

pub struct InputLabel<'a> {
    pub input_mode: &'a InputMode,
}

impl<'a> InputLabel<'a> {
    pub fn new(input_mode: &'a InputMode) -> InputLabel<'a> {
        InputLabel { input_mode }
    }
}

impl<'a> Widget for InputLabel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
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
}
