CREATE TABLE version_keywords (
    id SERIAL PRIMARY KEY,
    version_id INTEGER NOT NULL REFERENCES versions (id),
    keyword VARCHAR NOT NULL
);
