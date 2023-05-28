-- Your SQL goes here

CREATE TABLE IF NOT EXISTS users(
    id  UUID PRIMARY KEY  NOT NULL,
    email TEXT  NOT NULL,
    password TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

SELECT diesel_manage_updated_at('users');

CREATE TABLE IF NOT EXISTS cars (
    id UUID PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    price TEXT NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

SELECT diesel_manage_updated_at('cars');
