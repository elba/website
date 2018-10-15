CREATE TABLE groups
(
    id SERIAL PRIMARY KEY,
    group_name VARCHAR NOT NULL UNIQUE,
    group_name_origin VARCHAR NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users (id),
    created_at TIMESTAMP NOT NULL DEFAULT now()
);
