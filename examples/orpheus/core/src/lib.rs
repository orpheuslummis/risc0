use serde::{Deserialize, Serialize};

pub type Vote = u8;
pub type Votes = Vec<Vote>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub result: Vote,
}
