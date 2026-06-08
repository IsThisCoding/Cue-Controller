use iced::{
    Element, Length, Color,
    widget::{button, column, container, row, text},
};

use crate::ui::{Message, Session};

pub fn view(session: &Session) -> Element<'_, Message> {
    let mut content = column![
        row![
            text("ID").width(50),
            text("Name").width(Length::Fill),
            text("Note").width(Length::Fill),
        ].padding(10)
    ].spacing(5);

    for (id, cue) in session.workspace.cue_list.iter() {
        let is_selected = session.selected_cue_id == Some(*id);
        
        let row_content = row![
            text(id).width(50), 
            text(&cue.name).width(Length::Fill),
            text(&cue.note).width(Length::Fill),
        ].spacing(10);

        let mut btn = button(row_content)
            .width(Length::Fill)
            .on_press(Message::SelectCue(*id));

        if is_selected {
            btn = btn.style(|_theme, _status| button::Style {
                background: Some(iced::Background::Color(Color::from_rgb(0.2, 0.4, 0.8))),
                text_color: Color::WHITE,
                ..Default::default()
            });
        } else {
            btn = btn.style(|_theme, _status| button::Style {
                background: Some(iced::Background::Color(Color::from_rgb(0.15, 0.15, 0.15))),
                text_color: Color::WHITE,
                ..Default::default()
            });
        }

        content = content.push(btn);
    }

    container(content).width(Length::Fill).padding(10).into()
}
