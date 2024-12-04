use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct CityDataResponse {
    pub data: Vec<CityResponse>,
}

#[derive(Debug, Deserialize)]
pub struct CityResponse {
    pub code: String,
    pub name: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct City {
    pub code: String,
    pub name: String,
    pub province_id: String,
    pub created_at: DateTime<Utc>,  // Timestamp with time zone for creation date
    pub updated_at: DateTime<Utc>,  // Timestamp with time zone for last updated date
    pub deleted_at: Option<DateTime<Utc>>,  // Nullable timestamp for deleted date
}