use iced::{
    Border, Color, Element, Length,
    widget::{button, text},
};

use crate::ui::{Message, Session, styles::go_button_style};

pub fn view(session: &Session) -> Element<'_, Message> {
    button(text("GO").size(32).center())
        .on_press(Message::FireSelected)
        .on_press(Message::AddDefaultCue)
        .style(go_button_style)
        .into()
}
