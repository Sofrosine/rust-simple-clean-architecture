CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS subscriptions
(
    id                   UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    name                 VARCHAR(255)     NOT NULL UNIQUE,
    price                INT              NOT NULL,
    subscription_type_id UUID             NOT NULL,
    created_at           TIMESTAMP
                             WITH
                             TIME ZONE             DEFAULT NOW(),
    updated_at           TIMESTAMP
                             WITH
                             TIME ZONE             DEFAULT NOW(),
    deleted_at           TIMESTAMP
                             WITH
                             TIME ZONE    NULL,
    CONSTRAINT fk_subscription_type
        FOREIGN KEY (subscription_type_id) REFERENCES subscription_types(id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

