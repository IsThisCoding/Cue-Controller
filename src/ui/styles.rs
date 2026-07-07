use iced::Background;
use iced::Border;
use iced::Color;
use iced::Theme;
use iced::theme;
use iced::widget::MouseArea;
use iced::widget::button;
use iced::widget::button::StyleFn;
use iced::widget::container;
use iced::widget::text_input;

pub fn editable_cell_style(theme: &Theme, status: text_input::Status) -> text_input::Style {
    text_input::Style {
        background: Background::Color(Color::TRANSPARENT),
        border: Border {
            ..Default::default()
        },
        icon: Color::default(),
        placeholder: Color::default(),
        value: Color::default(),
        selection: Color::default(),
    }
}

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

pub fn selected_cue_style(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.2, 0.4, 0.8))),
        ..Default::default()
    }
}

pub fn default_cue_style(theme: &Theme) -> container::Style {
    container::Style {
        ..Default::default()
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
