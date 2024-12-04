use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct School {
    pub id: Uuid,               // UUID type for unique subscription identifier
    pub name: String,           // Subscription name, not null, unique
    pub address: String,           // Subscription name, not null, unique
    pub logo_path: String,           // Subscription name, not null, unique
    pub subscription_id: Option<Uuid>,               // UUID type for unique subscription identifier
    pub province_id: Option<String>,           // Subscription name, not null, unique
    pub city_id: Option<String>,           // Subscription name, not null, unique
    pub created_at: DateTime<Utc>,  // Timestamp with time zone for creation date
    pub updated_at: DateTime<Utc>,  // Timestamp with time zone for last updated date
    pub deleted_at: Option<DateTime<Utc>>,  // Nullable timestamp for deleted date
}