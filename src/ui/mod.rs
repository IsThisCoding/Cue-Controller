use crate::common::{
    CueId,
    cue::{Command, Cue},
    workspace::Workspace,
};
use crossbeam_channel::Sender;
use iced::{
    Border, Color, Element, Length, Task,
    advanced::{
        text::Paragraph,
        widget::operation::{self, focusable},
    },
    widget::{Id, column, container, operation::focus, row, text_editor, text_input},
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

#[derive(Clone, Debug, PartialEq)]
pub enum EditingField {
    Number,
    Name,
    PreWait,
    PostWait,
}

pub struct Session {
    pub command_tx: Sender<Command>,
    pub workspace: Workspace,
    pub selected_cue_id: Option<CueId>,
    pub active_cues: HashMap<CueId, Cue>,
    pub note_content_buffer: text_editor::Content,
    pub editing_field: Option<EditingField>,
}

pub enum CuePlaybackState {
    PreWaiting(Duration),
    Running { elapsed: Duration },
    Paused,
}

type TextInputId = String;

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
    SelectedCueNumberChanged(String),
    FocusTextInput(iced::widget::Id),
    SelectedCueNoteChanged(text_editor::Action),
    PendingCueNameChanged,
    NumberChanged(String),
    EditCue(CueId, EditingField),
}

impl Session {
    pub fn new(command_tx: Sender<Command>, workspace: Workspace) -> Self {
        Self {
            command_tx,
            workspace,
            selected_cue_id: None,
            editing_field: None,
            note_content_buffer: text_editor::Content::new(),
            active_cues: HashMap::new(),
        }
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EditCue(id, field) => {
                self.selected_cue_id = Some(id);
                self.editing_field = Some(field);
                Task::none()
            }
            Message::NumberChanged(new_num) => {
                if let Some(id) = self.selected_cue_id
                    && let Some(cue) = self.workspace.cue_list.get_mut(&id)
                {
                    cue.number = new_num;
                }
                Task::none()
            }
            Message::FocusTextInput(id) => {
                println!("This is like {:?}", id);
                focus(id)
            }
            Message::FireSelected => {
                println!("Firing selected cue!");
                Task::none()
            }
            Message::SelectCue(new_id) => {
                if let Some(old_id) = self.selected_cue_id
                    && let Some(cue) = self.workspace.cue_list.get_mut(&old_id)
                    && cue.name.is_empty()
                {
                    cue.name = format!("Cue {}", old_id);
                }
                self.selected_cue_id = Some(new_id);
                Task::none()
            }
            Message::AddDefaultCue => {
                self.workspace.add_cue("A cue");
                Task::none()
            }
            Message::SelectedCueNameChanged(name) => {
                if let Some(id) = self.selected_cue_id
                    && let Some(cue) = self.workspace.cue_list.get_mut(&id)
                {
                    cue.name = name;
                }
                Task::none()
            }
            Message::SelectedCueNoteChanged(action) => {
                self.note_content_buffer.perform(action.clone());

                if let Some(id) = self.selected_cue_id
                    && let Some(cue) = self.workspace.cue_list.get_mut(&id)
                // && note.is_edit()
                {
                    cue.note = self.note_content_buffer.text().trim().to_string();
                }
                Task::none()
            }
            _ => Task::none(),
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        views::control_deck::view(self)
    }
}
