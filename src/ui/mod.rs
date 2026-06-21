use crate::common::{
    CueId,
    cue::{Command, Cue},
    workspace::Workspace,
};
use crossbeam_channel::Sender;
use iced::{
    Border, Color, Element, Length,
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
pub mod views;

pub struct Session {
    pub command_tx: Sender<Command>,
    pub workspace: Workspace,
    pub selected_cue_id: Option<CueId>,
    pub active_cues: HashMap<CueId, Cue>,
}

pub enum CuePlaybackState {
    PreWaiting(Duration),
    Running { elapsed: Duration },
    Paused,
}

#[derive(Clone, Debug)]
pub enum Message {
    FireSelected,
    PauseAll,
    FadeAndStopAll,
    StopAll,
    PlayCue(CueId),
    StopCue(CueId),
    SelectCue(CueId),
    AddDefaultCue,
    SelectedCueNameChanged(String),
    PendingCueNameChanged,
}

impl Session {
    pub fn new(command_tx: Sender<Command>, workspace: Workspace) -> Self {
        Self {
            command_tx,
            workspace,
            selected_cue_id: None,
            active_cues: HashMap::new(),
        }
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::FireSelected => {
                println!("Firing selected cue!")
            }
            Message::SelectCue(id) => {
                self.selected_cue_id = Some(id);
                println!("{}", id);
            }
            Message::AddDefaultCue => {
                self.workspace.add_cue("A cue");
            }
            Message::SelectedCueNameChanged(name) => {
                if let Some(id) = self.selected_cue_id
                    && let Some(cue) = self.workspace.cue_list.get_mut(&id)
                {
                    cue.name = if name.is_empty() {
                        format!("Cue {}", id)
                    } else {
                        name
                    }
                }
            }
            _ => println!("Implement this!"),
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        views::control_deck::view(self)
    }
}
