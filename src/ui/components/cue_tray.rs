use iced::{
    Element,
    widget::{button, grid},
};

use crate::ui::{Message, Session};

pub fn view(session: &Session) -> Element<'_, Message> {
    button("♫").on_press(Message::A)
}
