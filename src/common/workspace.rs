use indexmap::IndexMap;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use uuid;

use crate::common::cue::Cue;

#[derive(Serialize, Deserialize)]
pub struct Workspace {
    pub name: String,
    pub id: uuid::Uuid,
    created_at: Timestamp,
    updated_at: Timestamp,
    pub max_id: u64,
    pub cue_list: IndexMap<u64, Cue>,
}

impl Workspace {
    pub fn new(name: String) -> Workspace {
        let now = Timestamp::now();
        Workspace {
            name,
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            max_id: 0,
            cue_list: IndexMap::new(),
        }
    }
}
