CREATE TABLE forms
(
    id          SERIAL PRIMARY KEY,
    name        text      NOT NULL,
    description text      NOT NULL,
    json_schema jsonb     NOT NULL,
    created_by  integer   NOT NULL REFERENCES users (id),
    created_at  TIMESTAMP NOT NULL DEFAULT now(),
    updated_at  TIMESTAMP NOT NULL DEFAULT now()
)
