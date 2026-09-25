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
pub struct SelectAssistantJourneyState {
    pub wrapper_state: GenericJourneyWidgetState<SelectState>,
    pub assistants: Vec<StoredAssistant>,
}

impl SelectAssistantJourneyState {
    pub fn new(input_state: TextInputState, assistants: Vec<StoredAssistant>) -> Self {
        let items = assistants
            .iter()
            .map(|a| format!("{} ({})", a.config.name, a.config.model))
            .collect();
        Self {
            wrapper_state: GenericJourneyWidgetState {
                input_state,
                child_state: SelectState::new(items),
            },
            assistants,
        }
    }
}

pub struct SelectAssistantJourneyWidget;

impl SelectAssistantJourneyWidget {
    pub fn handle_key_event(
        key: KeyEvent,
        state: &mut SelectAssistantJourneyState,
    ) -> JourneyOutcome<StoredAssistant> {
        let select_state = &mut state.wrapper_state.child_state;
        match key.code {
            KeyCode::Up => select_state.previous(),
            KeyCode::Down => select_state.next(),
            KeyCode::Enter => {
                let selected = select_state
                    .list_state
                    .selected()
                    .and_then(|i| state.assistants.get(i));
                if let Some(assistant) = selected {
                    return JourneyOutcome::Completed(assistant.clone());
                }
            }
            _ => {}
        }
        JourneyOutcome::Continue
    }
}

impl StatefulWidget for SelectAssistantJourneyWidget {
    type State = SelectAssistantJourneyState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let widget = GenericJourneyWidget {
            child: SelectWidget,
            title: "Select Assistant".to_owned(),
        };
        widget.render(area, buf, &mut state.wrapper_state);
    }
}
