use crate::InputMode;
use crate::shared::constants::USE_DEBUG;
use crate::shared::debug_logger::DebugLogger;
use chrono::{DateTime, Local};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{StatefulWidget, Widget};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Borders, Paragraph};
use std::sync::{Arc, Mutex};
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
    #[allow(dead_code)]
    pub debug_logger: Arc<Mutex<DebugLogger>>,
}

impl MessageListState {
    pub fn new(debug_logger: Arc<Mutex<DebugLogger>>) -> Self {
        Self {
            scroll_offset: 0,
            container_height: 0,
            text_height: 0,
            debug_logger,
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

pub struct MessageListWidget<'a> {
    messages: &'a Vec<ChatMessage>,
}

impl<'a> MessageListWidget<'a> {
    pub fn new(messages: &'a Vec<ChatMessage>) -> MessageListWidget<'a> {
        MessageListWidget { messages }
    }
}

impl<'a> StatefulWidget for MessageListWidget<'a> {
    type State = (&'a mut MessageListState, &'a mut InputMode);

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let (list_state, input_mode) = state;
        let [message_container, debug_container] =
            Layout::vertical([Constraint::Min(0), Constraint::Max(3)]).areas(area);

        let messages_block = Block::default()
            .borders(Borders::ALL)
            .title("Message History");

        let rendering_container_area = match USE_DEBUG {
            true => message_container,
            false => area,
        };

        let chat_area = messages_block.inner(rendering_container_area);
        messages_block.render(rendering_container_area, buf);

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
        list_state.text_height = text.height();
        list_state.container_height = chat_area.height as usize;

        if USE_DEBUG {
            let debug_container_block = Block::default().borders(Borders::ALL).title("Debug");
            let debug_area = debug_container_block.inner(debug_container);
            debug_container_block.render(debug_container, buf);

            Paragraph::new(Text::from(Line::from(format!(
                "flag: {:#?}; scroll offset: {:#?}; text height: {:#?}; chat height: {:#?}",
                list_state.has_room_for_bottom_scroll(),
                list_state.scroll_offset,
                list_state.text_height,
                list_state.container_height
            ))))
            .render(debug_area, buf);
        }

        if input_mode.clone() == InputMode::Editing && list_state.has_room_for_bottom_scroll() {
            list_state.scroll_offset = list_state.text_height - list_state.container_height;
        }

        let paragraph = Paragraph::new(text).scroll((list_state.scroll_offset as u16, 0));

        paragraph.render(chat_area, buf);
    }
}
