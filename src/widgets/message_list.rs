use chrono::{DateTime, Local};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Alignment, Line, StatefulWidget, Widget};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
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
}

impl MessageListState {
    pub fn new() -> Self {
        Self { scroll_offset: 0 }
    }

    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
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
        let messages_container = Block::default()
            .borders(Borders::ALL)
            .title("Message History");
        let chat_area = messages_container.inner(area);
        messages_container.render(area, buf);

        let mut lines: Vec<Line> = Vec::new();
        let max_bubble_width = (chat_area.width as f32 * 0.7) as usize;

        for message in self.messages {
            let wrapped_text = textwrap::wrap(&message.content, max_bubble_width);
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
                    let padding = " ".repeat(chat_area.width.saturating_sub(bubble_width as u16) as usize);
                    for line_content in bubble_lines_content {
                        let padded_line = format!("{}{}", padding, line_content);
                        lines.push(Line::from(padded_line).style(style));
                    }
                }
            }

            // Add a blank line between messages
            lines.push(Line::from(""));
        }

        let text = Text::from(lines);
        let paragraph = Paragraph::new(text).scroll((state.scroll_offset as u16, 0));

        paragraph.render(chat_area, buf);
    }
}
