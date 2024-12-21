use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::serde::ts_seconds::serialize as to_ts;
use chrono::{DateTime, Utc};
use rnglib::{Language, RNG};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheableItem {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    #[serde(deserialize_with = "from_ts")]
    #[serde(serialize_with = "to_ts")]
    pub created_at: DateTime<Utc>,
    #[serde(deserialize_with = "from_ts")]
    #[serde(serialize_with = "to_ts")]
    pub updated_at: DateTime<Utc>,
}

impl CacheableItem {
    pub fn id_as_str(&self) -> String {
        self.id.to_string()
    }

    fn new() -> CacheableItem {
        let rng = RNG::try_from(&Language::Elven).unwrap();
        CacheableItem {
            id: Uuid::new_v4(),
            first_name: rng.generate_name(),
            last_name: rng.generate_name(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}

impl Default for CacheableItem {
    fn default() -> Self {
        Self::new()
    }
}
