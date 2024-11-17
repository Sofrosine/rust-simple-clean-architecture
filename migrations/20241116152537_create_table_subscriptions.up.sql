CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS subscriptions
(
    id         UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    name       VARCHAR(255)     NOT NULL UNIQUE,
    price      INT              NOT NULL,
    created_at TIMESTAMP
                   WITH
                   TIME ZONE             DEFAULT NOW(),
    updated_at TIMESTAMP
                   WITH
                   TIME ZONE             DEFAULT NOW(),
    deleted_at TIMESTAMP
                   WITH
                   TIME ZONE             NULL
);

