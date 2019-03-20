CREATE TABLE version_keywords (
    version_id INTEGER PRIMARY KEY REFERENCES versions (id),
    keyword VARCHAR NOT NULL
);
