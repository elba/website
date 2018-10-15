CREATE TABLE packages
(
    id SERIAL PRIMARY KEY,
    group_id INTEGER NOT NULL REFERENCES groups (id),
    package_name VARCHAR NOT NULL,
    package_name_origin VARCHAR NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    created_at TIMESTAMP NOT NULL DEFAULT now(),

    CONSTRAINT unique_group_package UNIQUE (group_id, package_name)
);
