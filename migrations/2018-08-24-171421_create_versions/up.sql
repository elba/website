CREATE TABLE versions
(
    id SERIAL PRIMARY KEY,
    package_id INTEGER NOT NULL REFERENCES packages (id),
    semver VARCHAR NOT NULL,
    description VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT now(),

    CONSTRAINT unique_package_version UNIQUE (package_id, semver)
);
