use crate::shared::chat_action::{InputAction, SubmitData};
use crate::shared::text_input_state::{InputMode, TextInputState};
use crate::widgets::create_assistant_form::{
    CreateAssistantFormStep, CreateAssistantFormWidget, CreateAssistantFormWidgetState,
};
use crate::widgets::generic_journey::{GenericJourneyWidget, GenericJourneyWidgetState};
use crate::widgets::input::TextInputWidget;
use crate::widgets::select::SelectState;
use color_eyre::Result;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::prelude::StatefulWidget;

type WrapperState = GenericJourneyWidgetState<CreateAssistantFormWidgetState>;
 
#[derive(Clone)]
pub struct CreateAssistantJourneyState {
    pub wrapper_state: WrapperState,
}

impl CreateAssistantJourneyState {
    pub fn new(input_state: TextInputState, models: Vec<String>) -> Self {
        Self {
            wrapper_state: WrapperState {
                input_state: input_state.clone(),
                child_state: CreateAssistantFormWidgetState {
                    step: CreateAssistantFormStep::Name,
                    name: String::new(),
                    model: String::new(),
                    system_prompt_path: String::new(),
                    use_rag: false,
                    embedding_model: None,
                    rag_files_path: None,
                    input_state: input_state.clone(),
                    select_state: SelectState::new(models),
                    history: vec![],
                },
            },
        }
    }
}

pub struct CreateAssistantJourneyWidget;

impl CreateAssistantJourneyWidget {
    pub fn handle_key_event(key: KeyEvent, state: &mut CreateAssistantJourneyState) -> Result<()> {
        {
            let form_state = &mut state.wrapper_state.child_state;

            match form_state.step {
                CreateAssistantFormStep::Model => {
                    match key.code {
                        KeyCode::Up => form_state.select_state.previous(),
                        KeyCode::Down => form_state.select_state.next(),
                        KeyCode::Enter => {
                            if let Some(selected) = form_state.select_state.selected_item() {
                                form_state.model = selected.clone();
                                form_state.history.push(("Model".to_string(), selected));
                                form_state.step = CreateAssistantFormStep::SystemPromptPath;
                                // Reset input for next step if needed, though SystemPromptPath uses text input
                                form_state.input_state.reset();
                                form_state.input_state.input_mode = InputMode::Editing;
                                form_state.input_state.override_autocomplete_type =
                                    Some(crate::shared::autocomplete_state::ReferenceType::Filepath);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {
                    // Handle text input steps
                    let action = TextInputWidget::handle_key_event(key, &mut form_state.input_state);
                    if let InputAction::Submit(SubmitData::Text(content)) = action {
                        if !content.trim().is_empty() {
                            match form_state.step {
                                CreateAssistantFormStep::Name => {
                                    form_state.name = content.clone();
                                    form_state.history.push(("Name".to_string(), content));
                                    form_state.step = CreateAssistantFormStep::Model;
                                    // Switch to selection mode logic (handled above)
                                }
                                CreateAssistantFormStep::SystemPromptPath => {
                                    form_state.system_prompt_path = content.clone();
                                    form_state.history.push(("System Prompt".to_string(), content));
                                    form_state.step = CreateAssistantFormStep::UseRag;
                                    form_state.input_state.reset();
                                    form_state.input_state.override_autocomplete_type = None;
                                }
                                CreateAssistantFormStep::UseRag => {
                                    // Stop here as requested
                                    form_state.use_rag = content.to_lowercase() == "yes";
                                    form_state.history.push(("Use RAG".to_string(), content));
                                    // form_state.step = CreateAssistantFormStep::EmbeddingModel;
                                    // form_state.input_state.reset();
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        state.wrapper_state.input_state = state.wrapper_state.child_state.input_state.clone();
        Ok(())
    }
}

impl StatefulWidget for CreateAssistantJourneyWidget {
    type State = CreateAssistantJourneyState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let widget = GenericJourneyWidget {
            child: CreateAssistantFormWidget,
            title: "Create Assistant Journey".to_owned(),
        };
        widget.render(area, buf, &mut state.wrapper_state);
    }
}
