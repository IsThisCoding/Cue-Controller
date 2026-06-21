use iced::{
    Element,
    Length::{self, Fill},
    widget::{column, container, row},
};

use crate::ui::{
    Message, Session,
    components::{self, go_button, header},
    styles::container_style,
};

pub fn view(session: &Session) -> Element<'_, Message> {
    let go_button = container(go_button::view(session))
        .width(Length::Fixed(180.0))
        .height(Length::Fixed(80.0))
        .padding(4.0);
    let sidebar = container(components::active_cues_bar::view(session))
        .width(Length::Fixed(350.0))
        .height(Length::Fill)
        .style(container_style);
    let cue_table = container(components::cue_table::view(session)).height(Length::FillPortion(7));
    let header = container(header::view(session))
        .height(Length::FillPortion(1))
        .width(Length::Fill)
        .style(container_style);

    let main_layout = row![go_button, column![header, cue_table], sidebar];

    container(main_layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
