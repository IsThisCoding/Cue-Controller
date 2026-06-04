mod common;
mod engine;
mod ui;

use common::cue::Command;
use crossbeam_channel::unbounded;
use iced::Task;

use crate::{common::workspace::Workspace, ui::Session};

fn main() -> iced::Result {
    println!("Console started");

    let (command_tx, command_rx) = unbounded::<Command>();

    engine::spawn_backend(command_rx);

    command_tx
        .send(Command::PlayRawSound("testsound.mp3".to_string()))
        .unwrap();

    command_tx.send(Command::StopAll).unwrap();

    let default_workspace = Workspace::new("mock".to_string());

    iced::application(
        move || {
            (
                Session::new(command_tx.clone(), Workspace::new("mock".to_string())),
                Task::none(),
            )
        },
        Session::update,
        Session::view,
    )
    .title("Goomba")
    .run()
}
