CREATE TABLE sui_hei_puzzle_tokenize_cache (
    id SERIAL PRIMARY KEY,
    puzzle_id INTEGER NOT NULL UNIQUE,
    tokens JSONB NOT NULL
);

ALTER TABLE sui_hei_puzzle_tokenize_cache
    ADD FOREIGN KEY(puzzle_id) REFERENCES sui_hei_puzzle(id)
        DEFERRABLE INITIALLY DEFERRED;
