-- Create the `users` table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    salt TEXT NOT NULL,
    servers JSONB NOT NULL
);

-- Create the `posts` table
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    author TEXT NOT NULL,
    text TEXT NOT NULL,
    likes INTEGER DEFAULT 0,
    date BIGINT NOT NULL
);


