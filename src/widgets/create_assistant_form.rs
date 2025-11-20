use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{StatefulWidget, Widget};
use ratatui::widgets::Paragraph;

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

pub struct CreateAssistantFormWidgetState {
    pub step: CreateAssistantFormStep,
    pub name: String,
    pub model: String,
    pub system_prompt_path: String,
    pub use_rag: bool,
    pub embedding_model: Option<String>,
    pub rag_files_path: Option<String>,
}

pub struct CreateAssistantFormWidget;

impl StatefulWidget for CreateAssistantFormWidget {
    type State = CreateAssistantFormWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Render your form fields here based on state.step
        match state.step {
            CreateAssistantFormStep::Name => {
                Paragraph::new("Enter name").render(area, buf);
                
            }
            CreateAssistantFormStep::Model => {
                Paragraph::new("Select model").render(area, buf);
            }
            _ => {} // ... etc
        }
    }
}
