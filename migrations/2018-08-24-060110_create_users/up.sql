CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL,
    gh_id INTEGER NOT NULL UNIQUE,
    gh_name VARCHAR NOT NULL,
    gh_access_token VARCHAR NOT NULL,
    gh_avatar VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);
