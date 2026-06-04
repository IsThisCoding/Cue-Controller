use crate::common::{
    CueId,
    cue::{Command, Cue},
    workspace::Workspace,
};
use crossbeam_channel::Sender;
use iced::{
    Element, Length,
    widget::{column, container, row},
    window,
};
use indexmap::IndexMap;
use kira::PlaybackRate;
use std::{collections::HashMap, time::Duration};
use uuid::Uuid;

mod components;
mod persistence;
mod styles;

pub struct Session {
    pub command_tx: Sender<Command>,
    workspace: Workspace,
    selected_cue_id: Option<CueId>,
    pub active_states: HashMap<CueId, PlaybackRate>,
}

pub enum CuePlaybackState {
    PreWaiting(Duration),
    Running { elapsed: Duration },
    Paused,
}

pub enum Message {
    FireSelected,
    PauseAll,
    FadeAndStopAll,
    StopAll,
    PlayCue(CueId),
    StopCue(CueId),
}

impl Session {
    pub fn new(command_tx: Sender<Command>, workspace: Workspace) -> Self {
        Self {
            command_tx,
            workspace,
            selected_cue_id: None,
            active_states: HashMap::new(),
        }
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::FireSelected => {
                println!("Firing selected cue!")
            }
            _ => println!("Implement this!"),
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        let global_header = components::header::view(self);
        let cues_display = container("Main Workspace Cue Grid Panel Goes Here")
            .width(Length::FillPortion(20))
            .height(Length::FillPortion(7))
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        let active_cues = container("Active Cues")
            .width(Length::FillPortion(2))
            .height(Length::FillPortion(2));

        let row2 = row![cues_display, active_cues];

        let window_layout = column![global_header, row2];

        container(window_layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
