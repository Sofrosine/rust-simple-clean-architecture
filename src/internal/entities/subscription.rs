use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Subscription {
    pub id: Uuid,               // UUID type for unique subscription identifier
    pub name: String,           // Subscription name, not null, unique
    pub price: i32,             // Price of the subscription
    pub created_at: DateTime<Utc>,  // Timestamp with time zone for creation date
    pub updated_at: DateTime<Utc>,  // Timestamp with time zone for last updated date
    pub deleted_at: Option<DateTime<Utc>>,  // Nullable timestamp for deleted date
}
