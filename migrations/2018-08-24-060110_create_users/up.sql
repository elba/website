CREATE FUNCTION random_string(int4) RETURNS text AS $$
    SELECT (array_to_string(array(
        SELECT substr(
            'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789',
            floor(random() * 62)::int4 + 1,
            1
        ) FROM generate_series(1, $1)
    ), ''))
$$ LANGUAGE SQL;

CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    email VARCHAR,
    gh_id INTEGER NOT NULL UNIQUE,
    gh_name VARCHAR NOT NULL,
    gh_access_token VARCHAR NOT NULL,
    gh_avatar VARCHAR,
    token VARCHAR DEFAULT random_string(32) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    last_used_at TIMESTAMP NOT NULL
);
