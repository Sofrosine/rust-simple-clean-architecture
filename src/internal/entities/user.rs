use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_status")] // Must match the name of the SQL ENUM
#[sqlx(rename_all = "lowercase")] // Match the case of ENUM values in the database
pub enum UserStatus {
    Verified,
    Pending,
}
#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,               // UUID for unique SubscriptionType identifier
    pub name: String,           // Subscription type name
    pub email: String,           // Subscription type name
    pub password: String,           // Subscription type name
    pub phone_number: String,           // Subscription type name
    pub title: String,           // Subscription type name
    pub status: UserStatus,     // Subscription type name
    pub role_id: Uuid,           // Subscription type name
    pub school_id: Uuid,           // Subscription type name
    pub created_at: DateTime<Utc>,  // Timestamp with time zone for creation date
    pub updated_at: DateTime<Utc>,  // Timestamp with time zone for creation date
    pub deleted_at: Option<DateTime<Utc>>,  // Nullable timestamp for deletion date
}