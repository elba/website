CREATE TABLE package_groups
(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id),
    package_group_name VARCHAR NOT NULL UNIQUE,
    package_group_name_origin VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);