use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,  // Timestamp with time zone for creation date
    pub updated_at: DateTime<Utc>,  // Timestamp with time zone for last updated date
    pub deleted_at: Option<DateTime<Utc>>,  // Nullable timestamp for deleted date
}