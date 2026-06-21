use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, row, text, text_input};
use iced::{Color, Element, Length, widget::column};

use crate::ui::{Message, Session};

pub fn view(session: &Session) -> Element<'_, Message> {
    let Some(id) = session.selected_cue_id else {
        return text("No cue selected").into();
    };

    let Some(cue) = session.workspace.cue_list.get(&id) else {
        return text("No cue selected").into();
    };

    container(iced::widget::column![
        text_input("Title", &cue.name).on_input(Message::SelectedCueNameChanged),
        text_input("Note", &cue.note)
    ])
    .into()
}
