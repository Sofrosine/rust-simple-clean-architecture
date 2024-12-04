use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CreateSubscriptionTypeDto {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSubscriptionTypeDto {
    pub name: Option<String>,
}

