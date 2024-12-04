use serde::{Deserialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateSubscriptionDto {
    pub name: String,
    pub price: i32,
    pub subscription_type_id: Uuid
}

#[derive(Debug, Deserialize)]
pub struct UpdateSubscriptionDto {
    pub name: Option<String>,
    pub price: Option<i32>,
    pub subscription_type_id: Option<Uuid>
}

