mod services;
mod shared;
mod widgets;

use crate::shared::any_error::AnyError;
use crate::shared::chat_action::{InputAction, SubmitData};
use crate::shared::command::Command;
use crate::shared::constants::USE_DEBUG;
use crate::shared::debug_logger::DebugLogger;
use crate::shared::messages_storage::MessagesStorage;
use crate::shared::text_input_state::{InputMode, TextInputState};
use crate::shared::window_state::WindowState;
use crate::shared::window_state::WindowState::CommandFlow;
use crate::services::{assistant_configs, chat};
use crate::shared::assistant_config::{ActiveAssistant, StoredAssistant};
use crate::widgets::create_assistant_journey::{
    CreateAssistantJourneyState, CreateAssistantJourneyWidget,
};
use crate::widgets::generic_journey::JourneyOutcome;
use crate::widgets::remove_assistant_journey::{
    RemoveAssistantJourneyState, RemoveAssistantJourneyWidget,
};
use crate::widgets::select_assistant_journey::{
    SelectAssistantJourneyState, SelectAssistantJourneyWidget,
};
use crate::widgets::debug_block::DebugBlockWidget;
use crate::widgets::input::TextInputWidget;
use crate::widgets::input_label::InputLabelWidget;
use crate::widgets::message_list::{ChatRole, MessageListState, MessageListWidget};
use color_eyre::Result;
use ollama_rs::Ollama;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, layout::{Constraint, Layout}, style::Color,
    DefaultTerminal,
    Frame,
};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal).await;
    ratatui::restore();
    app_result
}

const THINKING_PLACEHOLDER: &str = "Thinking…";

/// App holds the state of the application
struct App {
    window_state: WindowState,
    messages: MessagesStorage,
    message_list_state: MessageListState,
    input_state: TextInputState,
    debug_logger: Arc<Mutex<DebugLogger>>,
    ollama: Ollama,
    assistant_journey_state: Option<CreateAssistantJourneyState>,
    select_assistant_state: Option<SelectAssistantJourneyState>,
    remove_assistant_state: Option<RemoveAssistantJourneyState>,
    active_assistant: Option<ActiveAssistant>,
    pending_reply: bool,
    models: Vec<String>,
}

impl App {
    fn new() -> Self {
        let debug_logger = Arc::new(Mutex::new(DebugLogger::new()));
        Self {
            window_state: WindowState::Default,
            messages: MessagesStorage::default(),
            message_list_state: MessageListState::new(debug_logger.clone()),
            input_state: TextInputState::new(&debug_logger),
            debug_logger,
            ollama: Ollama::default(),
            assistant_journey_state: None,
            select_assistant_state: None,
            remove_assistant_state: None,
            active_assistant: None,
            pending_reply: false,
            models: vec![],
        }
    }

    async fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), AnyError> {
        if let Ok(models) = self.ollama.list_local_models().await {
            self.models = models.iter().map(|m| m.name.clone()).collect();
        }

        loop {
            let state = self.window_state.clone();
            match state {
                WindowState::Default => {
                    terminal.draw(|frame| self.draw_chat(frame))?;
                }
                CommandFlow(cmd) => {
                    terminal.draw(|frame| self.draw_command_journey(frame, cmd))?;
                }
            }

            if self.pending_reply {
                self.pending_reply = false;
                self.fetch_reply().await;
                continue;
            }

            if let Event::Key(key) = event::read()? {
                match self.window_state {
                    WindowState::Default => match self.input_state.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('e') => {
                                self.input_state.input_mode = InputMode::Editing;
                            }
                            KeyCode::Char('q') => {
                                return Ok(());
                            }
                            KeyCode::Up => {
                                self.message_list_state.scroll_up();
                            }
                            KeyCode::Down => {
                                self.message_list_state.scroll_down();
                            }
                            _ => {}
                        },
                        InputMode::Editing if key.kind == KeyEventKind::Press => {
                            let action =
                                TextInputWidget::handle_key_event(key, &mut self.input_state);
                            match action {
                                InputAction::Submit(content) => {
                                    self.on_submit(content).await?;
                                }
                                InputAction::ExitEditingMode => {
                                    self.input_state.input_mode = InputMode::Normal;
                                }
                                InputAction::None => {}
                            }
                        }
                        InputMode::Editing => {}
                    },
                    CommandFlow(_) => self.handle_command_flow_key(key),
                }
            }
        }
    }

    fn draw_chat(&mut self, frame: &mut Frame) {
        let columns = if USE_DEBUG {
            [
                Constraint::Max(0),
                Constraint::Max(128),
                Constraint::Min(48),
            ]
        } else {
            [Constraint::Min(0), Constraint::Max(128), Constraint::Min(0)]
        };
        let [_, wrapper_area, debug_area] = Layout::horizontal(columns).areas(frame.area());

        let last_row_size = match &self.input_state.autocomplete_state.is_displayed() {
            true => 5,
            false => 1,
        };
        let vertical = Layout::vertical([
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Max(last_row_size),
        ]);
        // creating blocks from layout
        let [messages_area, input_area, help_area] = vertical.areas(wrapper_area);

        let input_mode = self.input_state.input_mode.clone();
        let input_label = InputLabelWidget::new(&input_mode);
        frame.render_stateful_widget(input_label, help_area, &mut self.input_state);

        let input_widget = match &self.active_assistant {
            Some(assistant) => TextInputWidget::new(true)
                .border_color(Color::Green)
                .title(format!(
                    "Input · {} ({})",
                    assistant.config.name, assistant.config.model
                )),
            None => TextInputWidget::new(true),
        };

        if self.input_state.input_mode == InputMode::Editing {
            frame.set_cursor_position(input_widget.cursor_position(input_area, &self.input_state));
        }

        let message_list_widget = MessageListWidget::new(&self.messages.entries);
        let mut compiled_message_list_state = (
            &mut self.message_list_state,
            &mut self.input_state.input_mode,
        );
        frame.render_stateful_widget(
            message_list_widget,
            messages_area,
            &mut compiled_message_list_state,
        );

        frame.render_stateful_widget(input_widget, input_area, &mut self.input_state);

        if USE_DEBUG {
            let debug_block = DebugBlockWidget::new(DebugLogger::get_messages(&self.debug_logger));
            frame.render_widget(debug_block, debug_area);
        }
    }

    fn draw_command_journey(&mut self, frame: &mut Frame, cmd: Command) {
        match cmd {
            Command::CreateAssistant(_) => {
                let widget = CreateAssistantJourneyWidget {};
                let mut state = self.assistant_journey_state.clone().unwrap_or(
                    CreateAssistantJourneyState::new(self.input_state.clone(), self.models.clone()),
                );
                frame.render_stateful_widget(widget, frame.area(), &mut state);
                self.assistant_journey_state = Some(state);
            }
            Command::SelectAssistant(_) => {
                if let Some(state) = self.select_assistant_state.as_mut() {
                    frame.render_stateful_widget(SelectAssistantJourneyWidget, frame.area(), state);
                }
            }
            Command::RemoveAssistant(_) => {
                if let Some(state) = self.remove_assistant_state.as_mut() {
                    frame.render_stateful_widget(RemoveAssistantJourneyWidget, frame.area(), state);
                }
            }
            Command::ListModels(_) => {}
        }
    }

    fn handle_command_flow_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Esc {
            self.close_command_flow();
            return;
        }

        let CommandFlow(cmd) = self.window_state.clone() else {
            return;
        };
        match cmd {
            Command::CreateAssistant(_) => {
                let Some(state) = self.assistant_journey_state.as_mut() else {
                    return;
                };
                if let Ok(JourneyOutcome::Completed(config)) =
                    CreateAssistantJourneyWidget::handle_key_event(key, state)
                {
                    let msg = match assistant_configs::save(&config) {
                        Ok(path) => format!(
                            "Assistant \"{}\" saved to {}",
                            config.name,
                            path.display()
                        ),
                        Err(err) => format!("Failed to save assistant: {err}"),
                    };
                    self.messages.append_message(ChatRole::App, msg);
                    self.close_command_flow();
                }
            }
            Command::SelectAssistant(_) => {
                let Some(state) = self.select_assistant_state.as_mut() else {
                    return;
                };
                if let JourneyOutcome::Completed(StoredAssistant { path, config }) =
                    SelectAssistantJourneyWidget::handle_key_event(key, state)
                {
                    match assistant_configs::load_system_prompt(&config) {
                        Ok(system_prompt) => {
                            self.messages.entries.clear();
                            self.message_list_state.scroll_offset = 0;
                            self.messages.append_message(
                                ChatRole::App,
                                format!("Active assistant: {} ({})", config.name, config.model),
                            );
                            self.active_assistant = Some(ActiveAssistant {
                                path,
                                config,
                                system_prompt,
                            });
                        }
                        Err(err) => self.messages.append_message(
                            ChatRole::App,
                            format!("Can't use assistant \"{}\", failed to read system prompt {err}", config.name),
                        ),
                    }
                    self.close_command_flow();
                }
            }
            Command::RemoveAssistant(_) => {
                let Some(state) = self.remove_assistant_state.as_mut() else {
                    return;
                };
                if let JourneyOutcome::Completed(assistants) =
                    RemoveAssistantJourneyWidget::handle_key_event(key, state)
                {
                    for assistant in assistants {
                        self.remove_assistant(assistant);
                    }
                    self.close_command_flow();
                }
            }
            Command::ListModels(_) => {}
        }
    }

    fn remove_assistant(&mut self, assistant: StoredAssistant) {
        let name = &assistant.config.name;
        let msg = match assistant_configs::remove(&assistant.path) {
            Ok(()) => {
                let was_active = self
                    .active_assistant
                    .as_ref()
                    .is_some_and(|active| active.path == assistant.path);
                if was_active {
                    self.active_assistant = None;
                }
                format!("Assistant \"{name}\" removed ({})", assistant.path.display())
            }
            Err(err) => format!("Failed to remove assistant \"{name}\": {err}"),
        };
        self.messages.append_message(ChatRole::App, msg);
    }

    fn load_assistants_for_picker(&mut self) -> Vec<StoredAssistant> {
        match assistant_configs::load_all() {
            Ok((assistants, errors)) => {
                for error in errors {
                    self.messages
                        .append_message(ChatRole::App, format!("Skipped broken config {error}"));
                }
                if assistants.is_empty() {
                    self.messages.append_message(
                        ChatRole::App,
                        "No assistants found. Create one with /create-assistant".to_string(),
                    );
                }
                assistants
            }
            Err(err) => {
                self.messages
                    .append_message(ChatRole::App, format!("Failed to load assistants: {err}"));
                vec![]
            }
        }
    }

    async fn fetch_reply(&mut self) {
        let Some(assistant) = &self.active_assistant else {
            return;
        };
        let result = chat::send(&self.ollama, assistant, &self.messages.entries).await;

        // replace the "Thinking…" bubble with the reply
        self.messages.entries.pop();
        match result {
            Ok(reply) => self.messages.append_message(ChatRole::Assistant, reply),
            Err(err) => self
                .messages
                .append_message(ChatRole::App, format!("Error: {err}")),
        }
    }

    fn close_command_flow(&mut self) {
        self.window_state = WindowState::Default;
        self.assistant_journey_state = None;
        self.select_assistant_state = None;
        self.remove_assistant_state = None;
    }

    async fn on_submit(&mut self, data: SubmitData) -> Result<(), AnyError> {
        match data {
            SubmitData::Text(content) => {
                if content.trim().is_empty() {
                    return Ok(());
                }
                self.messages.append_message(ChatRole::User, content);
                if self.active_assistant.is_some() {
                    self.messages
                        .append_message(ChatRole::App, THINKING_PLACEHOLDER.to_string());
                    self.pending_reply = true;
                } else {
                    self.messages.append_message(
                        ChatRole::App,
                        "Select an assistant with /select-assistant to chat.".to_string(),
                    );
                }
                Ok(())
            }
            SubmitData::Command(command) => match command {
                Command::ListModels(_) => match self.ollama.list_local_models().await {
                    Ok(models) => {
                        let list: Vec<String> = models
                            .iter()
                            .map(|m| format!("> {};", m.name.clone()))
                            .collect();
                        let msg = format!("Available models:\n\n{}", list.join("\n"));
                        self.messages.append_message(ChatRole::App, msg);
                        Ok(())
                    }
                    Err(err) => {
                        self.messages
                            .append_message(ChatRole::App, format!("{:?}", err));
                        Ok(())
                    }
                },
                Command::CreateAssistant(cmd) => {
                    self.window_state = CommandFlow(Command::CreateAssistant(cmd));
                    Ok(())
                }
                Command::SelectAssistant(cmd) => {
                    let assistants = self.load_assistants_for_picker();
                    if !assistants.is_empty() {
                        self.select_assistant_state = Some(SelectAssistantJourneyState::new(
                            self.input_state.clone(),
                            assistants,
                        ));
                        self.window_state = CommandFlow(Command::SelectAssistant(cmd));
                    }
                    Ok(())
                }
                Command::RemoveAssistant(cmd) => {
                    let assistants = self.load_assistants_for_picker();
                    if !assistants.is_empty() {
                        self.remove_assistant_state = Some(RemoveAssistantJourneyState::new(
                            self.input_state.clone(),
                            assistants,
                        ));
                        self.window_state = CommandFlow(Command::RemoveAssistant(cmd));
                    }
                    Ok(())
                }
            },
        }
    }
}
