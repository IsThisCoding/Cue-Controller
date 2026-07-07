use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, row, text, text_editor, text_input};
use iced::{Color, Element, Length, widget::column};

use crate::ui::{Message, Session};

pub fn view(session: &Session) -> Element<'_, Message> {
    //handles case where no cue is selected
    let Some(id) = session.selected_cue_id else {
        return container(text("No cue selected")).clip(true).into();
    };

    //handles case where selected cue is deleted
    let Some(cue) = session.workspace.cue_list.get(&id) else {
        return container(text("No cue selected")).clip(true).into();
    };

    container(iced::widget::column![
        container(text_input("Title", &cue.name).on_input(Message::SelectedCueNameChanged))
            .clip(true),
        text_editor(&session.note_content_buffer)
            .on_action(Message::SelectedCueNoteChanged)
            .height(Length::Fill)
            .wrapping(text::Wrapping::WordOrGlyph)
            .placeholder("Note")
    ])
    .into()
}
