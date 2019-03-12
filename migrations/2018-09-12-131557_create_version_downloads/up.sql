CREATE TABLE version_downloads
(
    id SERIAL PRIMARY KEY,
    version_id INTEGER NOT NULL REFERENCES versions (id),
    date Date NOT NULL DEFAULT current_date,
    downloads INTEGER NOT NULL DEFAULT 1,

    CONSTRAINT unique_version_date UNIQUE (version_id, date)
);
