use iced::Element;
use iced::widget::{button, container};
use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend, Tween, sound::static_sound::StaticSoundData,
};

struct AudioPlayer {
    filepath: String,
    manager: AudioManager,
    active_sound: Option<kira::sound::static_sound::StaticSoundHandle>,
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

        Self {
            filepath,
            manager,
            active_sound: None,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Play => {
                println!("Playing track {}!", self.filepath);
                // Handle the file loading and playback results inside the UI loop
                if let Ok(sound_data) = StaticSoundData::from_file(&self.filepath) {
                    self.active_sound = match self.manager.play(sound_data) {
                        Ok(val) => Some(val),
                        Err(e) => {
                            println!("Error Occured: {}", e);
                            None
                        }
                    }
                }
            }
            Message::Pause => {
                println!("Pause requested!");
                if let Some(to_pause) = &mut self.active_sound {
                    to_pause.pause(Tween {
                        ..Default::default()
                    });
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        use iced::widget::column;
        let play_button = button("Play").on_press(Message::Play);
        let pause_button = button("Pause").on_press(Message::Pause);

        let interface = column![play_button, pause_button];

        container(interface).into()
    }
}

fn main() -> iced::Result {
    // Iced manages instantiation, state updates, and the window view context
    iced::application(AudioPlayer::new, AudioPlayer::update, AudioPlayer::view).run()
}
