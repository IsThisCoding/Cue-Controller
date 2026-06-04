// src/ui/components/header.rs
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, row, text};
use iced::{Color, Element, Length};

use crate::ui::{Message, Session};

/// Renders the persistent global application header.
pub fn view(session: &Session) -> Element<'_, Message> {
    // 1. Left Section: Show Title
    let title_widget = text(&session.workspace.name).size(18);

    // 2. Center Section: Time Tracker placeholder (using Jiff timestamps later)
    let clock_widget = text("SYSTEM TIME: 17:23:36")
        .size(14)
        .font(iced::Font::MONOSPACE);

    // 3. Right Section: Telemetry Indicators
    // Hardcoded for now; later this reads from session.engine_connected state
    let engine_status = text("● ENGINE ONLINE")
        .size(12)
        .color(Color::from_rgb(0.0, 0.8, 0.0)); // Neon Green for healthy backend

    // Assemble the horizontal slice
    let header_row = row![title_widget, clock_widget, engine_status]
        .spacing(20)
        .align_y(Vertical::Center)
        .width(Length::Fill);

    // Wrap the row in a stylized, bounded container box
    container(header_row)
        .width(Length::Fill)
        .height(Length::Shrink)
        .padding([12, 20]) // 12px Top/Bottom, 20px Left/Right padding
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.12, 0.12, 0.14))), // Dark Charcoal
            border: iced::border::Border {
                color: Color::from_rgb(0.2, 0.2, 0.22), // Thin divider line underneath
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
}
