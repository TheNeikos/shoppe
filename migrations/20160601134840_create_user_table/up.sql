CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL,
    password_hash VARCHAR NOT NULL,
    name VARCHAR NOT NULL
)
