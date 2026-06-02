use iced::widget::{Column, button, container, text};
use iced::{Alignment, Element, Length};
use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend, Tween, sound::static_sound::StaticSoundData,
};

struct AudioPlayer {
    filepath: String,
    manager: AudioManager,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Play,
    Pause,
}

impl AudioPlayer {
    fn new() -> Self {
        let filepath = "/home/michael/Cabaret-Theatre/Big-Chair/Sound Effects/213925__diboz__pistol_riccochet.ogg".to_string();
        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
            .expect("Failed to initialize AudioManager");

        Self { filepath, manager }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Play => {
                println!("Playing track {}!", self.filepath);
                // Handle the file loading and playback results inside the UI loop
                if let Ok(sound_data) = StaticSoundData::from_file(&self.filepath) {
                    if let Ok(mut sound) = self.manager.play(sound_data) {
                        let _ = sound.set_panning(-1.0, Tween::default());
                    }
                }
            }
            Message::Pause => {
                println!("Pause requested!");
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let play_button = button("Play").on_press(Message::Play);

        let content = Column::new()
            .push(text("Show Control Sandbox"))
            .push(play_button)
            .spacing(20)
            .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn main() -> iced::Result {
    // Iced manages instantiation, state updates, and the window view context
    iced::run(
        "Barebones Cue Player",
        AudioPlayer::update,
        AudioPlayer::view,
    )
}
