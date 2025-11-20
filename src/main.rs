mod services;
mod shared;
mod widgets;

use crate::shared::autocomplete_state::AutocompleteState;
use crate::shared::chat_action::{InputAction, SubmitData};
use crate::shared::command::Command;
use crate::shared::constants::USE_DEBUG;
use crate::shared::debug_logger::DebugLogger;
use crate::shared::text_input_state::TextInputState;
use crate::shared::window_state::WindowState;
use crate::shared::window_state::WindowState::CommandFlow;
use crate::widgets::debug_block::DebugBlock;
use crate::widgets::input::TextInput;
use crate::widgets::input_label::InputLabel;
use crate::widgets::message_list::{ChatMessage, ChatRole, MessageList, MessageListState};
use chrono::Local;
use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Position},
};
use std::sync::{Arc, Mutex};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

/// App holds the state of the application
struct App {
    window_state: WindowState,
    /// History of recorded messages
    messages: Vec<ChatMessage>,
    /// State for the message list widget
    message_list_state: MessageListState,
    /// State for the input
    input_state: TextInputState,

    debug_logger: Arc<Mutex<DebugLogger>>,
}

#[derive(Debug, PartialEq, Eq)]
enum InputMode {
    Normal,
    Editing,
}

impl App {
    fn new() -> Self {
        let debug_logger = Arc::new(Mutex::new(DebugLogger::new()));
        Self {
            window_state: WindowState::Default,
            messages: vec![ChatMessage {
                content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc vitae orci sed dui luctus cursus ac non odio. Etiam id faucibus lectus, sit amet tincidunt ipsum. Nunc malesuada bibendum felis id rutrum. Maecenas magna nulla, scelerisque ac augue id, fermentum interdum diam. Fusce nec laoreet lectus. Etiam id hendrerit nisi. Quisque scelerisque dui eu dictum lobortis. Fusce turpis metus, pulvinar ut justo pellentesque, faucibus convallis nulla. Fusce non porta ipsum.

Phasellus rhoncus orci urna, ac ullamcorper ipsum malesuada nec. Aenean ac malesuada lorem. Phasellus ut nulla erat. Praesent eget velit ut sapien sagittis sagittis vehicula vel turpis. Cras sed est fringilla, porta justo sit amet, dictum nisl. Quisque sodales tellus nec cursus elementum. Morbi dapibus sagittis eros, tempor varius lacus laoreet viverra. Proin lacinia nisi metus, eget vulputate quam posuere et. Quisque egestas nibh vitae pretium sodales. Phasellus convallis nec purus ut feugiat. Fusce molestie tincidunt sapien, eget tristique augue pretium sed. Curabitur imperdiet quam vel hendrerit viverra. Nam sit amet nibh eu est elementum pulvinar vitae vitae elit. Maecenas tempus rhoncus vehicula. Aenean vitae commodo dolor. Sed quis lacinia magna.

Praesent suscipit nulla eget est aliquet, vehicula rutrum nunc gravida. Etiam bibendum eget magna eu finibus. Vestibulum auctor, nunc sit amet gravida sagittis, ligula est dignissim justo, vitae cursus mi orci ut mi. Duis ac fringilla arcu. Morbi interdum felis sed diam dapibus, id congue diam posuere. Nam laoreet nisi eget porta rutrum. Nulla interdum ultrices risus sit amet porta. Fusce laoreet ex eget sem lacinia, sed aliquet quam cursus. Nunc rhoncus vel tortor non laoreet. Maecenas lobortis ligula tellus, eget vestibulum velit ultrices a. Quisque maximus erat velit, vitae vehicula magna rhoncus eget. Ut auctor, ante in fringilla pellentesque, neque libero convallis dui, a ornare nulla justo ut leo. Integer in lobortis ipsum, eget pharetra velit. Praesent aliquet placerat mattis.".to_string(),
                timestamp: Local::now(),
                role: ChatRole::App,
            },
                           ChatMessage {
                               content: "asdlkansdkjabndjkasLorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc vitae orci sed dui luctus cursus ac non odio. Etiam id faucibus lectus, sit amet tincidunt ipsum. Nunc malesuada bibendum felis id rutrum. Maecenas magna nulla, scelerisque ac augue id, fermentum interdum diam. Fusce nec laoreet lectus. Etiam id hendrerit nisi. Quisque scelerisque dui eu dictum lobortis. Fusce turpis metus, pulvinar ut justo pellentesque, faucibus convallis nulla. Fusce non porta ipsum.".to_string(),
                               timestamp: Local::now(),
                               role: ChatRole::User,
                           }],
            message_list_state: MessageListState::new(debug_logger.clone()),
            input_state: TextInputState {
                prefix: String::from(" > | "),
                input: String::new(),
                character_index: 0,
                debug_logger: debug_logger.clone(),
                autocomplete_state: AutocompleteState::new(),
            },
            debug_logger,
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
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
                match self.message_list_state.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            self.message_list_state.input_mode = InputMode::Editing;
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
                        let action = TextInput::handle_key_event(key, &mut self.input_state);
                        match action {
                            InputAction::Submit(content) => {
                                self.on_submit(content);
                            }
                            InputAction::ExitEditingMode => {
                                self.message_list_state.input_mode = InputMode::Normal;
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

        let input_label = InputLabel::new(&self.message_list_state.input_mode);
        frame.render_stateful_widget(input_label, help_area, &mut self.input_state);

        match self.message_list_state.input_mode {
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

        let message_list_widget = MessageList::new(&self.messages);
        frame.render_stateful_widget(
            message_list_widget,
            messages_area,
            &mut self.message_list_state,
        );

        let input_widget = TextInput::new(&self.message_list_state.input_mode);
        frame.render_stateful_widget(input_widget, input_area, &mut self.input_state);

        if USE_DEBUG {
            let debug_block = DebugBlock::new(DebugLogger::get_messages(&self.debug_logger));
            frame.render_widget(debug_block, debug_area);
        }
    }

    fn on_submit(&mut self, data: SubmitData) {
        match data {
            SubmitData::Text(content) => {
                if !content.trim().is_empty() {
                    self.messages.push(ChatMessage {
                        content,
                        timestamp: Local::now(),
                        role: ChatRole::User,
                    });
                }
            }
            SubmitData::Command(command) => match command {
                Command::ListModels(_) => {}
                Command::CreateAssistant(cmd) => {
                    self.window_state = CommandFlow(Command::CreateAssistant(cmd));
                }
            },
        }
    }
}
