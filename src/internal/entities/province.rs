use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct ProvinceDataResponse {
    pub data: Vec<ProvinceResponse>,
}

#[derive(Debug, Deserialize)]
pub struct ProvinceResponse {
    pub code: String,
    pub name: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Province {
    pub code: String,
    pub name: String,
    pub created_at: DateTime<Utc>,  // Timestamp with time zone for creation date
    pub updated_at: DateTime<Utc>,  // Timestamp with time zone for last updated date
    pub deleted_at: Option<DateTime<Utc>>,  // Nullable timestamp for deleted date
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct ProvinceFromTable {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,  // Timestamp with time zone for creation date
    pub updated_at: DateTime<Utc>,  // Timestamp with time zone for last updated date
    pub deleted_at: Option<DateTime<Utc>>,  // Nullable timestamp for deleted date
}