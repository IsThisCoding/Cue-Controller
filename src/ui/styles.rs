use iced::Border;
use iced::Color;
use iced::Theme;
use iced::theme;
use iced::widget::button;
use iced::widget::button::StyleFn;
use iced::widget::container;

pub fn go_button_style(theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Pressed => button::Style {
            text_color: Color::from_rgba(0.0, 0.3, 0.0, 0.7),
            ..Default::default()
        },
        _ => button::Style {
            text_color: Color::from_rgb(0.0, 1.0, 0.0),
            border: Border {
                color: Color::from_rgb(0.0, 0.6, 0.0),
                width: 2.0,
                ..Default::default()
            },
            ..Default::default()
        },
    }
}

pub fn container_style(theme: &Theme) -> container::Style {
    container::Style {
        border: Border {
            color: (theme.palette().primary),
            width: (3.0),
            ..Default::default()
        },
        ..Default::default()
    }
}
