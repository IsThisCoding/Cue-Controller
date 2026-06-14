use iced::{
    Element, Length,
    widget::{container, row},
};

use crate::ui::{
    Message, Session,
    components::{self, go_button},
};

pub fn view(session: &Session) -> Element<'_, Message> {
    let sidebar = components::active_cues_bar::view(session);
    let cue_table = components::cue_table::view(session);

    let main_layout = row![go_button::view(session), cue_table, sidebar];

    container(main_layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
