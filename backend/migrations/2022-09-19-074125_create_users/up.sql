-- Your SQL goes here
CREATE TABLE users (
    user_id uuid DEFAULT gen_random_uuid (),
    username VARCHAR PRIMARY KEY,
    password VARCHAR,
    registerd_time TIMESTAMP
)
