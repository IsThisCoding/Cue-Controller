use iced::{
    Color, Element,
    Length::{self, FillPortion},
    advanced::Widget,
    widget::{self, button, column, container, mouse_area, row, text, text_input},
};

use crate::{
    common::CueId,
    ui::{
        EditingField, Message, Session,
        styles::{default_cue_style, editable_cell_style, selected_cue_style},
    },
};

fn is_editing(session: &Session, cue_id: CueId, field: EditingField) -> bool {
    session.selected_cue_id == Some(cue_id) && session.editing_field == Some(field)
}

fn editable_cell<'a>(
    value: &'a str,
    is_editing: bool,
    width: Length,
    on_input: impl Fn(String) -> Message + 'a,
    on_double_click: Message,
    id: widget::Id,
) -> Element<'a, Message> {
    if is_editing {
        let id = widget::Id::unique();
        let input = text_input("", value)
            .on_input(on_input)
            .on_submit(Message::StopEditing)
            .id(id)
            .width(width)
            .size(12)
            .style(editable_cell_style);
        container(input).height(20).into()
    } else {
        mouse_area(text(value).width(width).size(12).height(20))
            .on_double_click(on_double_click)
            .into()
    }
}

pub fn view(session: &Session) -> Element<'_, Message> {
    let mut content = column![
        row![
            text("Num").width(50),
            text("Cue").width(Length::Fill).center(),
            text("Target").width(80).center(),
            text("Pre Wait").width(80).center(),
            text("Runtime").width(80).center(),
            text("Post Wait").width(80).center(),
        ]
        .padding(5)
    ]
    .spacing(5);

    for (index, cue) in session.workspace.cue_list.iter() {
        let is_selected = session.selected_cue_id == Some(cue.id);

        let num_id = widget::Id::unique();
        let number_cell = editable_cell(
            &cue.number,
            is_editing(session, cue.id, EditingField::Number),
            Length::Fixed(50.0),
            Message::SelectedCueNumberChanged,
            Message::EditCue(cue.id, EditingField::Number, num_id.clone()),
            num_id,
        );

        let name_id = widget::Id::unique();
        let name_cell = editable_cell(
            &cue.name,
            is_editing(session, cue.id, EditingField::Name),
            Length::Fill,
            Message::SelectedCueNameChanged,
            Message::EditCue(cue.id, EditingField::Name, name_id.clone()),
            name_id,
        );

        let row_content: Element<'_, Message> = container(
            mouse_area(row![number_cell, name_cell]).on_press(Message::SelectCue(cue.id)),
        )
        .width(Length::Fill)
        .style(move |theme| {
            if is_selected {
                selected_cue_style(theme)
            } else {
                default_cue_style(theme)
            }
        })
        .into();

        content = content.push(row_content);
    }

    container(content).width(Length::Fill).padding(10).into()
}
