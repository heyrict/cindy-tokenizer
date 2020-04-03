use crate::schema::sui_hei_puzzle_tokenize_cache;
use crate::tokenize::Token;
use diesel::pg::data_types::{PgDate, PgTimestamp};
use serde::{Deserialize, Serialize};

type Timestamptz = PgTimestamp;
type Date = PgDate;

#[derive(Queryable)]
pub struct Puzzle {
    pub id: i32,
    pub title: String,
    pub yami: i32,
    pub genre: i32,
    pub content: String,
    pub solution: String,
    pub created: Timestamptz,
    pub modified: Timestamptz,
    pub status: i32,
    pub memo: String,
    pub user_id: i32,
    pub anonymous: bool,
    pub dazed_on: Date,
    pub grotesque: bool,
}

#[derive(Queryable)]
pub struct Dialogue {
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub is_good: bool,
    pub is_true: bool,
    pub created: Timestamptz,
    pub answered_time: Option<Timestamptz>,
    pub puzzle_id: i32,
    pub user_id: i32,
    pub answer_edit_times: i32,
    pub question_edit_times: i32,
    pub qno: i32,
    pub modified: Timestamptz,
}

#[derive(Serialize, Deserialize)]
pub struct DialogueTokens {
    pub id: i32,
    pub tokens: Vec<Token>,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct PuzzleTokenCache {
    pub id: i32,
    pub puzzle_id: i32,
    pub tokens: serde_json::Value,
}

impl PuzzleTokenCache {
    pub fn to_tokens(self) -> Result<Vec<DialogueTokens>, serde_json::Error> {
        serde_json::from_value(self.tokens)
    }
}

#[derive(Insertable)]
#[table_name = "sui_hei_puzzle_tokenize_cache"]
pub struct NewPuzzleTokenCache {
    pub puzzle_id: i32,
    pub tokens: serde_json::Value,
}

impl NewPuzzleTokenCache {
    pub fn new(puzzle_id: i32, tokens: &Vec<DialogueTokens>) -> Self {
        Self {
            puzzle_id,
            tokens: serde_json::to_value(tokens).unwrap(),
        }
    }
}
