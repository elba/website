CREATE TABLE package_owners
(
    id SERIAL PRIMARY KEY,
    package_id INTEGER NOT NULL REFERENCES packages (id),
    user_id INTEGER NOT NULL REFERENCES users (id),

    CONSTRAINT unique_package_owner UNIQUE (package_id, user_id)
);
