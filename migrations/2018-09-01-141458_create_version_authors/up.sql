CREATE TABLE version_authors
(
    id SERIAL PRIMARY KEY,
    version_id INTEGER NOT NULL REFERENCES versions (id),
    name VARCHAR NOT NULL
);
