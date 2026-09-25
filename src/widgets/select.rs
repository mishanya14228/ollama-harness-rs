use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{StatefulWidget, Stylize};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};

#[derive(Clone)]
pub struct SelectState {
    pub list_state: ListState,
    pub items: Vec<String>,
}

impl SelectState {
    pub fn new(items: Vec<String>) -> Self {
        let mut list_state = ListState::default();
        if !items.is_empty() {
            list_state.select(Some(0));
        }
        Self { items, list_state }
    }

    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn selected_item(&self) -> Option<String> {
        self.list_state
            .selected()
            .and_then(|i| self.items.get(i).cloned())
    }
}

pub struct SelectWidget;

impl StatefulWidget for SelectWidget {
    type State = SelectState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let items: Vec<ListItem> = state
            .items
            .iter()
            .map(|i| ListItem::new(i.clone()))
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(ratatui::style::Style::default().reversed());

        StatefulWidget::render(list, area, buf, &mut state.list_state);
    }
}
