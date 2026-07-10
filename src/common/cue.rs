use crate::common::CueId;
use serde::{Deserialize, Serialize};

pub enum Command {
    FireNext,
    FireCue(CueId),
    StopCue(CueId),
    StopAll,
    PlayRawSound(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl default for CueTypeConfig::Audio {}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cue {
    pub id: CueId,
    pub name: String,
    pub note: String,
    pub number: String,
    pub config: CueTypeConfig,
    pub metadata: CueMetadata,
}
