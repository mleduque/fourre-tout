CREATE TABLE models (
    id BIGSERIAL PRIMARY KEY,
    version BIGSERIAL UNIQUE NOT NULL,
    name CHARACTER VARYING NOT NULL,
    old_version BIGINT REFERENCES models,
    body JSONB NOT NULL,
    published TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    first_published TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
)