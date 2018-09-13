CREATE TABLE readmes
(
    version_id INTEGER PRIMARY KEY REFERENCES versions (id),
    textfile VARCHAR NOT NULL
);
