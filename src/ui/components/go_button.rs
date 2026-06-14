use iced::{Border, Color, Element, Length, widget::button};

use crate::ui::{Message, Session};

pub fn view(session: &Session) -> Element<'_, Message> {
    button("GO")
        .on_press(Message::FireSelected)
        .style(go_button_style)
        .height(Length::Fixed(200.0))
        .width(Length::Fixed(200.0))
        .into()
}

fn go_button_style(theme: &iced::Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Pressed => button::Style {
            text_color: Color::from_rgba(0.0, 0.3, 0.0, 0.7),
            ..Default::default()
        },
        _ => button::Style {
            text_color: Color::from_rgb(0.0, 1.0, 0.0),
            border: Border {
                color: Color::from_rgb(0.0, 0.6, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
    }
}
