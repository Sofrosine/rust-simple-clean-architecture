CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS schools
(
    id              UUID PRIMARY KEY         NOT NULL DEFAULT (uuid_generate_v4()),
    name            VARCHAR(255)             NOT NULL,
    address         TEXT,
    logo_path       TEXT,
    subscription_id UUID                     NULL,
    province_id     VARCHAR(255),
    city_id         VARCHAR(255),
    created_at      TIMESTAMP WITH TIME ZONE          DEFAULT NOW(),
    updated_at      TIMESTAMP WITH TIME ZONE          DEFAULT NOW(),
    deleted_at      TIMESTAMP WITH TIME ZONE NULL,
    CONSTRAINT fk_subscription
        FOREIGN KEY (subscription_id) REFERENCES subscriptions (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE,
    CONSTRAINT fk_province
        FOREIGN KEY (province_id) REFERENCES provinces (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE,
    CONSTRAINT fk_city
        FOREIGN KEY (city_id) REFERENCES cities (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);
