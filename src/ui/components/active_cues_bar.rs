use iced::{
    Element, Length,
    widget::{column, container, progress_bar, text},
};
use indexmap::IndexMap;

use crate::{
    common::{CueId, cue::Cue},
    ui::{Message, Session},
};

pub fn view(session: &Session) -> Element<'_, Message> {
    let mut content = column![text("Active Cues")];

    for (_id, cue_state) in &session.active_cues {
        let row_ui = column![
            text(&cue_state.name).size(14),
            progress_bar(0.0..=1.0, 0.5).girth(4.0)
        ]
        .spacing(5);

        content = content.push(row_ui).padding([5, 0]);
    }

    container(content).width(Length::Fill).padding(10).into()
}
