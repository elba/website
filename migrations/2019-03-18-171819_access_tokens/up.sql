CREATE FUNCTION random_string(int4) RETURNS text AS $$
    SELECT (array_to_string(array(
        SELECT substr(
            'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789',
            floor(random() * 62)::int4 + 1,
            1
        ) FROM generate_series(1, $1)
    ), ''))
$$ LANGUAGE SQL;

CREATE TABLE access_tokens
(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id),
    token VARCHAR DEFAULT random_string(32) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    last_used_at TIMESTAMP
);
