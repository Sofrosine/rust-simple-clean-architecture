CREATE TABLE
    IF NOT EXISTS roles
(
    id         UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    name       VARCHAR(255)     NOT NULL UNIQUE,
    created_at TIMESTAMP
                   WITH
                   TIME ZONE             DEFAULT NOW(),
    updated_at TIMESTAMP
                   WITH
                   TIME ZONE             DEFAULT NOW(),
    deleted_at TIMESTAMP
                   WITH
                   TIME ZONE    NULL
)
