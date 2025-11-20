use crate::shared::text_input_state::TextInputState;
use crate::widgets::input::TextInputWidget;
use crate::widgets::select::{SelectState, SelectWidget};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{StatefulWidget, Widget};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;

#[derive(Clone, PartialEq)]
pub enum CreateAssistantFormStep {
    Name,
    Model,
    SystemPromptPath,
    UseRag,
    EmbeddingModel,
    RagFilesPath,
    Confirmation,
    Completed,
}

#[derive(Clone)]
pub struct CreateAssistantFormWidgetState {
    pub step: CreateAssistantFormStep,
    pub name: String,
    pub model: String,
    pub system_prompt_path: String,
    pub use_rag: bool,
    pub embedding_model: Option<String>,
    pub rag_files_path: Option<String>,

    pub input_state: TextInputState,
    pub select_state: SelectState,
    pub history: Vec<(String, String)>, // (Label, Value)
}

pub struct CreateAssistantFormWidget;

impl StatefulWidget for CreateAssistantFormWidget {
    type State = CreateAssistantFormWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let history_len = state.history.len() as u16;
        let current_step_height = match state.step {
            CreateAssistantFormStep::Model => 10, // Give space for list
            _ => 3,
        };

        let [history_area, current_step_area] = Layout::vertical([
            Constraint::Length(history_len),
            Constraint::Length(current_step_height),
        ])
        .areas(area);

        // Render History
        for (i, (label, value)) in state.history.iter().enumerate() {
            let line = Line::from(vec![
                format!("{}: ", label).into(),
                value.clone().white(),
            ]);
            let area = Rect::new(history_area.x, history_area.y + i as u16, history_area.width, 1);
            Paragraph::new(line).render(area, buf);
        }

        // Render Current Step
        match state.step {
            CreateAssistantFormStep::Name => {
                self.render_input_step("Enter name: ", current_step_area, buf, &mut state.input_state);
            }
            CreateAssistantFormStep::Model => {
                let [label_area, list_area] = Layout::vertical([
                    Constraint::Length(1),
                    Constraint::Min(0),
                ]).areas(current_step_area);
                
                Paragraph::new("Select model:").render(label_area, buf);
                SelectWidget.render(list_area, buf, &mut state.select_state);
            }
            CreateAssistantFormStep::SystemPromptPath => {
                self.render_input_step("Enter system prompt path: ", current_step_area, buf, &mut state.input_state);
            }
             CreateAssistantFormStep::UseRag => {
                 // For now, just reuse input for Yes/No or implement another select
                 // The user asked to work UNTIL "use rag" step, so we can stop here or implement basic input
                 self.render_input_step("Enable RAG? (Yes/No): ", current_step_area, buf, &mut state.input_state);
            }
            _ => {}
        }
    }
}

impl CreateAssistantFormWidget {
    fn render_input_step(&self, label_text: &str, area: Rect, buf: &mut Buffer, input_state: &mut TextInputState) {
        let wrapped_text = textwrap::wrap(label_text, 2000);
        let text_width = wrapped_text.iter().map(|s| s.len()).max().unwrap_or(0);
        
        let [label_container, input_container] = Layout::horizontal([
            Constraint::Length((text_width) as u16),
            Constraint::Fill(1),
        ])
        .areas(area);

        Paragraph::new(label_text).render(label_container, buf);
        TextInputWidget::new(false).render(input_container, buf, input_state);
    }
}
