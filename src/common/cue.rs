use crate::common::CueId;
use serde::{Deserialize, Serialize};

pub enum Command {
    FireNext,
    FireCue(CueId),
    StopCue(CueId),
    StopAll,
    PlayRawSound(String),
}

#[derive(Serialize, Deserialize)]
pub enum CueTypeConfig {
    Audio {
        filepath: String,
        output_device: String,
    },
    Video {
        filepath: String,
        output_screen: String,
    },
}

#[derive(Serialize, Deserialize)]
pub struct CueMetadata {
    mock_data: String,
}

impl Default for CueMetadata {
    fn default() -> Self {
        CueMetadata {
            mock_data: "Please figure out meaningful metadata to have!".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Cue {
    pub id: CueId,
    pub name: String,
    pub note: String,
    pub number: String,
    // pub config: CueTypeConfig,
    // pub metadata: CueMetadata,
}
