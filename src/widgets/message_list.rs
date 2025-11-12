use crate::InputMode;
use chrono::{DateTime, Local};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Line, StatefulWidget, Widget};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph};
use std::iter::once;
use textwrap;

pub enum ChatRole {
    User,
    App,
}

pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
    pub timestamp: DateTime<Local>,
}

pub struct MessageListState {
    pub scroll_offset: usize,
    pub container_height: usize,
    pub text_height: usize,
    pub input_mode: InputMode,
}

impl MessageListState {
    pub fn new() -> Self {
        Self {
            scroll_offset: 0,
            container_height: 0,
            text_height: 0,
            input_mode: InputMode::Normal,
        }
    }

    fn has_room_for_bottom_scroll(&mut self) -> bool {
        self.text_height > self.container_height
    }

    pub fn scroll_up(&mut self) {
        if self.has_room_for_bottom_scroll() {
            self.scroll_offset = self.scroll_offset.saturating_sub(1);
        }
    }

    pub fn scroll_down(&mut self) {
        if self.has_room_for_bottom_scroll() {
            let can_scroll = self.scroll_offset + self.container_height < self.text_height;
            if can_scroll {
                self.scroll_offset = self.scroll_offset.saturating_add(1);
            }
        }
    }
}

pub struct MessageList<'a> {
    messages: &'a Vec<ChatMessage>,
}

impl<'a> MessageList<'a> {
    pub fn new(messages: &'a Vec<ChatMessage>) -> MessageList<'a> {
        MessageList { messages }
    }
}

impl<'a> StatefulWidget for MessageList<'a> {
    type State = MessageListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [message_container, debug_container] =
            Layout::vertical([Constraint::Min(0), Constraint::Max(3)]).areas(area);

        let messages_block = Block::default()
            .borders(Borders::ALL)
            .title("Message History");
        let chat_area = messages_block.inner(message_container);
        messages_block.render(message_container, buf);

        let debug_container_block = Block::default().borders(Borders::ALL).title("Debug");
        let debug_area = debug_container_block.inner(debug_container);
        debug_container_block.render(debug_container, buf);

        let mut lines: Vec<Line> = Vec::new();
        let max_bubble_width = (chat_area.width as f32 * 0.7) as usize;

        // let messages_starting_with_none: Vec<Option<&ChatMessage>> =
        //     once(None).chain(self.messages.iter().map(Some)).collect();

        for message in self.messages {
            // let prev = &messages[0];
            // let message = &messages[1];
            let content = format!(
                "[{}]: {}",
                &message.timestamp.format("%H:%M:%S"),
                &message.content
            );
            let wrapped_text = textwrap::wrap(content.as_str(), max_bubble_width);
            let text_width = wrapped_text.iter().map(|s| s.len()).max().unwrap_or(0);
            let bubble_width = text_width + 4; // text + 2 spaces + 2 border chars

            let style = match message.role {
                ChatRole::User => Style::default().fg(Color::Cyan),
                ChatRole::App => Style::default().fg(Color::Gray),
            };

            // --- Build the bubble content (borders and text) ---
            let mut bubble_lines_content: Vec<String> = Vec::new();
            bubble_lines_content.push(format!("╭{}╮", "─".repeat(text_width + 2)));
            for line in wrapped_text {
                bubble_lines_content.push(format!("│ {:width$} │ ", line, width = text_width));
            }
            bubble_lines_content.push(format!("╰{}╯", "─".repeat(text_width + 2)));

            // --- Add padding for alignment ---
            match message.role {
                ChatRole::App => {
                    // Left-aligned, no padding needed
                    for line_content in bubble_lines_content {
                        lines.push(Line::from(line_content).style(style));
                    }
                }
                ChatRole::User => {
                    // Right-aligned, add padding to the left
                    let padding =
                        " ".repeat(chat_area.width.saturating_sub(bubble_width as u16) as usize);
                    for line_content in bubble_lines_content {
                        let padded_line = format!("{}{}", padding, line_content);
                        lines.push(Line::from(padded_line).style(style));
                    }
                }
            }

            // Add a blank line between messages
            // lines.push(Line::from(""));
        }

        let text = Text::from(lines);
        state.text_height = text.height();
        state.container_height = chat_area.height as usize;

        Paragraph::new(Text::from(Line::from(format!(
            "flag: {:#?}; scroll offset: {:#?}; text height: {:#?}; chat height: {:#?}",
            state.has_room_for_bottom_scroll(),
            state.scroll_offset,
            state.text_height,
            state.container_height
        ))))
        .render(debug_area, buf);

        if state.input_mode == InputMode::Editing && state.has_room_for_bottom_scroll() {
            state.scroll_offset = state.text_height - state.container_height;
        }

        let paragraph = Paragraph::new(text).scroll((state.scroll_offset as u16, 0));

        paragraph.render(chat_area, buf);
    }
}
