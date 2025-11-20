use crate::shared::text_input_state::TextInputState;
use crate::widgets::create_assistant_form::{
    CreateAssistantFormStep, CreateAssistantFormWidget, CreateAssistantFormWidgetState,
};
use crate::widgets::generic_journey::{GenericJourneyWidget, GenericJourneyWidgetState};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::StatefulWidget;

type WrapperState = GenericJourneyWidgetState<CreateAssistantFormWidgetState>;

pub struct CreateAssistantJourneyState {
    wrapper_state: WrapperState,
}

impl CreateAssistantJourneyState {
    pub fn new(input_state: TextInputState) -> Self {
        Self {
            wrapper_state: WrapperState {
                input_state,
                child_state: CreateAssistantFormWidgetState {
                    step: CreateAssistantFormStep::Name,
                    name: String::new(),
                    model: String::new(),
                    system_prompt_path: String::new(),
                    use_rag: false,
                    embedding_model: None,
                    rag_files_path: None,
                },
            },
        }
    }
}

pub struct CreateAssistantJourneyWidget;

impl StatefulWidget for CreateAssistantJourneyWidget {
    type State = CreateAssistantJourneyState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let widget = GenericJourneyWidget {
            child: CreateAssistantFormWidget,
            title: "Create Assistant Journey".to_owned(),
        };
        widget.render(area, buf, &mut state.wrapper_state);

        // let widget = GenericJourneyWidget {
        //     child: self,
        //     title: "hello".to_owned(),
        // };
        //
        //
        // // let mut state = CreateAssistantJourneyState::default();
        // // let mut state = self.wrapper_state.clone();
        // widget.render(area, buf, &mut state.wrapper_state);

        // let [_, container, _] = Layout::horizontal([
        //     Constraint::Fill(1),
        //     Constraint::Max(64),
        //     Constraint::Fill(1),
        // ])
        // .areas(area);
        //
        // let journey_block = Block::default()
        //     .borders(Borders::ALL)
        //     .title("Create assistant journey");
        //
        // let journey_area = journey_block.inner(container);
        // journey_block.render(container, buf);
    }
}
