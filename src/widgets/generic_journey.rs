use crate::shared::assistant_config::AssistantConfig;
use crate::shared::text_input_state::TextInputState;
use crate::widgets::input_label::InputLabelWidget;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{StatefulWidget, Widget};
use ratatui::widgets::{Block, Borders};

pub enum JourneyOutcome {
    Continue,
    Completed(AssistantConfig),
}

pub struct GenericJourneyWidget<W: StatefulWidget> {
    pub title: String,
    pub child: W,
}

#[derive(Clone)]
pub struct GenericJourneyWidgetState<ChildState> {
    pub input_state: TextInputState,
    pub child_state: ChildState,
}

impl<W: StatefulWidget> StatefulWidget for GenericJourneyWidget<W> {
    type State = GenericJourneyWidgetState<W::State>;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [_, horizontal_container, _] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Max(64),
            Constraint::Fill(1),
        ])
        .areas(area);

        let [_, container, input_label_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(2), Constraint::Min(8)])
                .areas(horizontal_container);

        let journey_block = Block::default().borders(Borders::ALL).title(self.title);
        let journey_area = journey_block.inner(container);

        journey_block.render(container, buf);
        self.child.render(journey_area, buf, &mut state.child_state);

        let input_mode = state.input_state.input_mode.clone();

        let input_label = InputLabelWidget::new(&input_mode);
        input_label.render(input_label_area, buf, &mut state.input_state);

        // input_label_area.render_stat(input_label, buf);
        // frame.render_stateful_widget(input_label, help_area, &mut self.input_state);
        // input_label_area
    }
}
