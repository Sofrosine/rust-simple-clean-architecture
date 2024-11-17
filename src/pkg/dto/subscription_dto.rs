use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CreateSubscriptionDto {
    pub name: String,
    pub price: i32,
}

