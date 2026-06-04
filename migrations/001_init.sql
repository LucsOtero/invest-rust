CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL
);

CREATE TABLE investments (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id),
    asset TEXT NOT NULL,
    quantity FLOAT NOT NULL,
    price FLOAT NOT NULL
);
