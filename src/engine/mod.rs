mod audio;
mod video;

use crate::common::cue::Command;
use crossbeam_channel::Receiver;

pub fn spawn_backend(rx: Receiver<Command>) {
    std::thread::spawn(move || {
        println!("Backend thread started");

        while let Ok(command) = rx.recv() {
            match command {
                Command::FireNext => {
                    println!("Firing next Cue!")
                }
                Command::FireCue(cue_id) => {
                    println!("Firing cue with ID '{}'", cue_id)
                }
                Command::StopCue(cue_id) => {
                    println!("Stopping cue with ID '{}'", cue_id)
                }
                Command::StopAll => {
                    println!("Stopping all Cues!")
                }
                Command::PlayRawSound(filepath) => {
                    println!("Playing sound at {}", filepath);
                }
            }
        }
        println!("Backend thread closed");
    });
}
