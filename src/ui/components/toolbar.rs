use iced::{
    Element, Length, Color,
    widget::{button, container, row},
    alignment::Vertical,
};

use crate::ui::{Message, Session};

pub fn view(_session: &Session) -> Element<'_, Message> {
    let go_btn = button("GO")
        .padding([10, 30])
        .on_press(Message::FireSelected);

    let stop_all_btn = button("Stop All")
        .padding([10, 20])
        .on_press(Message::StopAll);
        
    let fade_stop_all_btn = button("Fade & Stop All")
        .padding([10, 20])
        .on_press(Message::FadeAndStopAll);

    let pause_all_btn = button("Pause All")
        .padding([10, 20])
        .on_press(Message::PauseAll);

    let toolbar_row = row![go_btn, stop_all_btn, fade_stop_all_btn, pause_all_btn]
        .spacing(15)
        .align_y(Vertical::Center);

    container(toolbar_row)
        .width(Length::Fill)
        .padding(10)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.15, 0.15, 0.17))),
            border: iced::border::Border {
                color: Color::from_rgb(0.2, 0.2, 0.22),
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
}
