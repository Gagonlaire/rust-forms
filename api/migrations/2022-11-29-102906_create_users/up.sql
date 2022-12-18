CREATE TABLE users
(
    id         SERIAL PRIMARY KEY,
    username   TEXT      NOT NULL,
    email      TEXT      NOT NULL UNIQUE,
    password   TEXT      NOT NULL,
    validated  BOOLEAN   NOT NULL DEFAULT FALSE,
    admin      BOOLEAN   NOT NULL DEFAULT FALSE,
    form_ids   INTEGER[] NOT NULL DEFAULT '{}',
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
)
