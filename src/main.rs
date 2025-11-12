mod widgets;

use crate::widgets::message_list::{ChatMessage, ChatRole, MessageList, MessageListState};
use chrono::Local;
use color_eyre::Result;
use ratatui::widgets::BorderType;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout, Position},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Paragraph},
    DefaultTerminal,
    Frame,
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
    /// Current value of the input box
    input: String,
    /// Position of cursor in the editor area.
    character_index: usize,
    /// History of recorded messages
    messages: Vec<ChatMessage>,
    /// State for the message list widget
    message_list_state: MessageListState,
}

#[derive(Debug, PartialEq, Eq)]
enum InputMode {
    Normal,
    Editing,
}

impl App {
    fn new() -> Self {
        Self {
            input: String::new(),
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
            character_index: 0,
            message_list_state: MessageListState::new(),
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn submit_message(&mut self) {
        let content = self.input.clone();
        let timestamp = Local::now();
        self.messages.push(ChatMessage {
            content,
            timestamp,
            role: ChatRole::User,
        });
        self.input.clear();
        self.reset_cursor();
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
                    InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => self.submit_message(),
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Esc => self.message_list_state.input_mode = InputMode::Normal,
                        _ => {}
                    },
                    InputMode::Editing => {}
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let [_, wrapper_area, _] =
            Layout::horizontal([Constraint::Min(0), Constraint::Max(128), Constraint::Min(0)])
                .areas(frame.area());
        let vertical = Layout::vertical([
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Length(1),
            // Constraint::
        ]);
        // creating blocks from layout
        let [messages_area, input_area, help_area] = vertical.areas(wrapper_area);

        // rendering text block widget
        let (msg, style) = match self.message_list_state.input_mode {
            InputMode::Normal => (
                vec![
                    "Press ".into(),
                    "q".bold(),
                    " to exit, ".into(),
                    "e".bold(),
                    " to start editing.".bold(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    "Press ".into(),
                    "Esc".bold(),
                    " to stop editing, ".into(),
                    "Enter".bold(),
                    " to record the message".into(),
                ],
                Style::default(),
            ),
        };
        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, help_area);

        // rendering input
        let input_prefix = " > | ";
        let input_text = format!("{}{}", input_prefix, self.input);
        let input = Paragraph::new(input_text.as_str())
            .style(match self.message_list_state.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title("Input"),
            );
        frame.render_widget(input, input_area);

        match self.message_list_state.input_mode {
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            InputMode::Normal => {}

            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            #[allow(clippy::cast_possible_truncation)]
            InputMode::Editing => frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                input_prefix.len() as u16 + input_area.x + self.character_index as u16 + 1,
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
    }
}
