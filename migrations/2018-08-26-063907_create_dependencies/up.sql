CREATE TABLE dependencies
(
    id SERIAL PRIMARY KEY,
    version_id INTEGER NOT NULL REFERENCES versions (id),
    package_id INTEGER NOT NULL REFERENCES packages (id),
    version_req VARCHAR NOT NULL,
    
    CONSTRAINT unique_version_dependency UNIQUE (version_id, package_id)
);
