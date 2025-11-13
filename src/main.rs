mod widgets;

use crate::widgets::input::{InputAction, TextInput, TextInputState};
use crate::widgets::input_label::InputLabel;
use crate::widgets::message_list::{ChatMessage, ChatRole, MessageList, MessageListState};
use chrono::Local;
use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Position},
    style::Stylize,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

/// App holds the state of the application
struct App {
    /// History of recorded messages
    messages: Vec<ChatMessage>,
    /// State for the message list widget
    message_list_state: MessageListState,
    /// State for the input
    input_state: TextInputState,
}

#[derive(Debug, PartialEq, Eq)]
enum InputMode {
    Normal,
    Editing,
}

impl App {
    fn new() -> Self {
        Self {
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
            message_list_state: MessageListState::new(),
            input_state: TextInputState {
                prefix: String::from(" > | "),
                input: String::new(),
                character_index: 0,
            }
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

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
                                if !content.trim().is_empty() {
                                    self.messages.push(ChatMessage {
                                        content,
                                        timestamp: Local::now(),
                                        role: ChatRole::User,
                                    });
                                }
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

    fn draw(&mut self, frame: &mut Frame) {
        let [_, wrapper_area, _] =
            Layout::horizontal([Constraint::Min(0), Constraint::Max(128), Constraint::Min(0)])
                .areas(frame.area());

        let last_row_size = match self.message_list_state.input_mode {
            InputMode::Normal => 1,
            InputMode::Editing => 5,
        };
        let vertical = Layout::vertical([
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Max(last_row_size),
        ]);
        // creating blocks from layout
        let [messages_area, input_area, help_area] = vertical.areas(wrapper_area);

        let input_label = InputLabel::new(&self.message_list_state.input_mode, &self.input_state);
        frame.render_widget(input_label, help_area);

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
        frame.render_stateful_widget(input_widget, input_area, &mut self.input_state)
    }
}
