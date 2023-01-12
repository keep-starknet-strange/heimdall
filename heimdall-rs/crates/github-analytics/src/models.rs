use chrono::prelude::NaiveDateTime;
#[derive(Debug)]
pub struct Interaction {
    pub created_at: NaiveDateTime,
    pub ended_at: Option<NaiveDateTime>,
    pub author: String,
    pub interaction_type: String,
}
