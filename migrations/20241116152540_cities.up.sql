CREATE TABLE IF NOT EXISTS cities
(
    id          VARCHAR(255) PRIMARY KEY NOT NULL,
    name        VARCHAR(255)             NOT NULL UNIQUE,
    province_id VARCHAR(255)             NOT NULL,
    created_at  TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at  TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at  TIMESTAMP WITH TIME ZONE NULL,
    CONSTRAINT fk_province
        FOREIGN KEY (province_id) REFERENCES provinces(id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);
