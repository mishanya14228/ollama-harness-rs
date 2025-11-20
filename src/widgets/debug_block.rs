use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Widget};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use textwrap::{Options, wrap};


pub struct DebugBlockWidget {
    debug_messages: Vec<String>,
}

impl DebugBlockWidget {
    pub fn new(debug_messages: Vec<String>) -> Self {
        Self { debug_messages }
    }
}

impl Widget for DebugBlockWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let lines: Vec<Line> = self
            .debug_messages
            .iter()
            .map(|x| Line::from(x.as_str()))
            .collect();

        let debug_block = Block::default()
            .borders(Borders::ALL)
            .title("Debug messages");

        let block_area = debug_block.inner(area);
        debug_block.render(area, buf);

        let width = block_area.width as usize;
        let mut visual_lines: Vec<Line> = Vec::new();
        for msg in &self.debug_messages {
            let opts = Options::new(width).break_words(true); // matches ratatui wrapping

            for wrapped in wrap(msg, opts) {
                visual_lines.push(Line::from(wrapped.into_owned()));
            }
        }

        let total_rows = visual_lines.len();
        let visible_rows = block_area.height as usize;
        let scroll = total_rows.saturating_sub(visible_rows);

        let debug_paragraph = Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .scroll((scroll as u16, 0));
        debug_paragraph.render(block_area, buf);
    }
}
