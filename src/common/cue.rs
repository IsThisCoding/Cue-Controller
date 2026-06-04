use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

type CueId = u64;

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
    pub config: CueTypeConfig,
    pub metadata: CueMetadata,
}

static CUE_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

pub fn next_cue_id() -> u64 {
    CUE_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub fn seed_cue_id_counter(val: u64) {
    CUE_ID_COUNTER.store(val, Ordering::SeqCst);
}
