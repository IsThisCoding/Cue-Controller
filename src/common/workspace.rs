use indexmap::IndexMap;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use uuid;

use crate::common::{CueId, cue::Cue};

#[derive(Serialize, Deserialize)]
pub struct Workspace {
    pub name: String,
    pub id: uuid::Uuid,
    created_at: Timestamp,
    next_cue_id: CueId,
    updated_at: Timestamp,
    pub cue_list: IndexMap<CueId, Cue>,
}

impl Workspace {
    pub fn new(name: String) -> Workspace {
        let now = Timestamp::now();
        Workspace {
            name,
            next_cue_id: 1,
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            cue_list: IndexMap::new(),
        }
    }

    pub fn peek_next_cue_id(&self) -> CueId {
        self.next_cue_id
    }

    pub fn add_cue(&mut self, name: &str) {
        let id = self.next_cue_id;
        self.next_cue_id += 1;
        self.cue_list.insert(
            id,
            Cue {
                id,
                name: name.to_string(),
                note: "".to_string(),
            },
        );
    }
}
