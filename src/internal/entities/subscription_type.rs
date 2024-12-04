use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::internal::entities::subscription::Subscription;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct SubscriptionType {
    pub id: Uuid,               // UUID for unique SubscriptionType identifier
    pub name: String,           // Subscription type name
    pub created_at: DateTime<Utc>,  // Timestamp with time zone for creation date
    pub updated_at: DateTime<Utc>,  // Timestamp with time zone for creation date
    pub deleted_at: Option<DateTime<Utc>>,  // Nullable timestamp for deletion date
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct SubscriptionTypeResponse {
    pub id: Uuid,               // UUID for unique SubscriptionType identifier
    pub name: String,           // Subscription type name
    pub created_at: DateTime<Utc>,  // Timestamp with time zone for creation date
    pub updated_at: DateTime<Utc>,  // Timestamp with time zone for creation date
    pub deleted_at: Option<DateTime<Utc>>,  // Nullable timestamp for deletion date
    pub subscriptions: Vec<Subscription>,
}