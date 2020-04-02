use crate::models::*;
use crate::tokenize::{tokenize, Poc, Token};
use diesel::prelude::*;
use diesel::result::Error;

pub fn find_puzzle_dialogues(
    puzzle_id_p: i32,
    conn: &PgConnection,
) -> Result<Vec<Dialogue>, Error> {
    use crate::schema::sui_hei_dialogue::dsl::*;

    sui_hei_dialogue
        .filter(puzzle_id.eq(puzzle_id_p))
        .load::<Dialogue>(conn)
}

pub fn find_puzzle_token_cache_by_id(
    id_p: i32,
    conn: &PgConnection,
) -> Result<Option<PuzzleTokenCache>, Error> {
    use crate::schema::sui_hei_puzzle_tokenize_cache::dsl::*;

    sui_hei_puzzle_tokenize_cache
        .filter(puzzle_id.eq(id_p))
        .first::<PuzzleTokenCache>(conn)
        .optional()
}

pub fn insert_puzzle_token_cache(
    puzzle_id: i32,
    tokens: &Vec<Vec<Token>>,
    conn: &PgConnection,
) -> Result<PuzzleTokenCache, Error> {
    use crate::schema::sui_hei_puzzle_tokenize_cache;

    let new_cache = NewPuzzleTokenCache::new(puzzle_id, tokens);

    diesel::insert_into(sui_hei_puzzle_tokenize_cache::table)
        .values(&new_cache)
        .get_result(conn)
}

pub fn get_puzzle_tokens(puzzle_id_p: i32, conn: &PgConnection) -> Result<Vec<Vec<Token>>, Error> {
    let cache = find_puzzle_token_cache_by_id(puzzle_id_p, &conn)?;
    match cache {
        Some(cache) => cache
            .to_tokens()
            .map_err(|e| Error::DeserializationError(Box::new(e))),
        None => {
            // Tokenize tokens from dialogues
            let dialogues = find_puzzle_dialogues(puzzle_id_p, &conn)?;
            let tokens_list = dialogues
                .iter()
                .map(|dialogue| tokenize(&dialogue.question))
                .map(|tokens| {
                    // Filtering useless tokens
                    tokens
                        .into_iter()
                        .filter(|token| match token.get_poc() {
                            Poc::Noun => match token.get_poc_detail() {
                                "数" => false,
                                _ => true,
                            },
                            Poc::Verb => true,
                            _ => false,
                        })
                        .collect::<Vec<Token>>()
                })
                .collect::<Vec<Vec<Token>>>();

            // Cache the tokenized tokens
            insert_puzzle_token_cache(puzzle_id_p, &tokens_list, &conn)?;

            Ok(tokens_list)
        }
    }
}
