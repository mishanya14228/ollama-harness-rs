mod services;
mod shared;
mod widgets;

use crate::shared::any_error::AnyError;
use crate::shared::autocomplete_state::AutocompleteState;
use crate::shared::chat_action::{InputAction, SubmitData};
use crate::shared::command::Command;
use crate::shared::constants::USE_DEBUG;
use crate::shared::debug_logger::DebugLogger;
use crate::shared::messages_storage::MessagesStorage;
use crate::shared::text_input_state::{InputMode, TextInputState};
use crate::shared::window_state::WindowState;
use crate::shared::window_state::WindowState::CommandFlow;
use crate::widgets::debug_block::DebugBlockWidget;
use crate::widgets::input::TextInputWidget;
use crate::widgets::input_label::InputLabelWidget;
use crate::widgets::message_list::{ChatRole, MessageListState, MessageListWidget};
use color_eyre::Result;
use ollama_rs::Ollama;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Position},
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

/// App holds the state of the application
struct App {
    window_state: WindowState,
    messages: MessagesStorage,
    message_list_state: MessageListState,
    input_state: TextInputState,
    debug_logger: Arc<Mutex<DebugLogger>>,
    ollama: Ollama,
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
        }
    }

    async fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), AnyError> {
        loop {
            match &self.window_state {
                WindowState::Default => {
                    terminal.draw(|frame| self.draw_chat(frame))?;
                }
                CommandFlow(cmd) => {
                    print!("{:?}", cmd);
                    terminal.clear()?;
                }
            }

            if let Event::Key(key) = event::read()? {
                match self.input_state.input_mode {
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
                        let action = TextInputWidget::handle_key_event(key, &mut self.input_state);
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

        match self.input_state.input_mode {
            InputMode::Normal => {}
            #[allow(clippy::cast_possible_truncation)]
            InputMode::Editing => frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                self.input_state.prefix.len() as u16
                    + input_area.x
                    + self.input_state.character_index as u16
                    + 1,
                // Move one line down, from the border to the input line
                input_area.y + 1,
            )),
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

        let input_widget = TextInputWidget::new(&input_mode);
        frame.render_stateful_widget(input_widget, input_area, &mut self.input_state);

        if USE_DEBUG {
            let debug_block = DebugBlockWidget::new(DebugLogger::get_messages(&self.debug_logger));
            frame.render_widget(debug_block, debug_area);
        }
    }

    async fn on_submit(&mut self, data: SubmitData) -> Result<(), AnyError> {
        match data {
            SubmitData::Text(content) => {
                if !content.trim().is_empty() {
                    self.messages.append_message(ChatRole::User, content);
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
            },
        }
    }
}
