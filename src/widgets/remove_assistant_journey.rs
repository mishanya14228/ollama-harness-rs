use crate::shared::assistant_config::StoredAssistant;
use crate::shared::text_input_state::TextInputState;
use crate::widgets::generic_journey::{
    GenericJourneyWidget, GenericJourneyWidgetState, JourneyOutcome,
};
use crate::widgets::select::{SelectState, SelectWidget};
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::prelude::StatefulWidget;

#[derive(Clone)]
pub struct RemoveAssistantJourneyState {
    pub wrapper_state: GenericJourneyWidgetState<SelectState>,
    pub assistants: Vec<StoredAssistant>,
    pub marked: Vec<bool>,
}

impl RemoveAssistantJourneyState {
    pub fn new(input_state: TextInputState, assistants: Vec<StoredAssistant>) -> Self {
        let marked = vec![false; assistants.len()];
        let mut state = Self {
            wrapper_state: GenericJourneyWidgetState {
                input_state,
                child_state: SelectState::new(vec![]),
            },
            assistants,
            marked,
        };
        state.wrapper_state.child_state = SelectState::new(state.items());
        state
    }

    fn items(&self) -> Vec<String> {
        self.assistants
            .iter()
            .zip(&self.marked)
            .map(|(a, &marked)| {
                let checkbox = if marked { "[x]" } else { "[ ]" };
                format!("{checkbox} {} ({})", a.config.name, a.config.model)
            })
            .collect()
    }
}

pub struct RemoveAssistantJourneyWidget;

impl RemoveAssistantJourneyWidget {
    pub fn handle_key_event(
        key: KeyEvent,
        state: &mut RemoveAssistantJourneyState,
    ) -> JourneyOutcome<Vec<StoredAssistant>> {
        match key.code {
            KeyCode::Up => state.wrapper_state.child_state.previous(),
            KeyCode::Down => state.wrapper_state.child_state.next(),
            KeyCode::Char(' ') => {
                if let Some(i) = state.wrapper_state.child_state.list_state.selected()
                    && let Some(marked) = state.marked.get_mut(i)
                {
                    *marked = !*marked;
                    state.wrapper_state.child_state.items = state.items();
                }
            }
            KeyCode::Enter => {
                let to_remove: Vec<StoredAssistant> = state
                    .assistants
                    .iter()
                    .zip(&state.marked)
                    .filter(|(_, marked)| **marked)
                    .map(|(a, _)| a.clone())
                    .collect();
                if !to_remove.is_empty() {
                    return JourneyOutcome::Completed(to_remove);
                }
            }
            _ => {}
        }
        JourneyOutcome::Continue
    }
}

impl StatefulWidget for RemoveAssistantJourneyWidget {
    type State = RemoveAssistantJourneyState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let widget = GenericJourneyWidget {
            child: SelectWidget,
            title: "Remove Assistants · Space: mark · Enter: remove".to_owned(),
        };
        widget.render(area, buf, &mut state.wrapper_state);
    }
}
