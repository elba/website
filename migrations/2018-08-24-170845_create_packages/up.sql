CREATE TABLE packages
(
    id SERIAL PRIMARY KEY,
    package_group_id INTEGER NOT NULL REFERENCES package_groups (id),
    package_name VARCHAR NOT NULL,
    package_name_origin VARCHAR NOT NULL,
    description VARCHAR,
    updated_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),

    CONSTRAINT unique_group_package UNIQUE (package_group_id, package_name)
);