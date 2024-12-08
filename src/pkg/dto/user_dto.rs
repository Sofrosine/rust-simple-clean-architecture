use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::internal::entities::user::UserStatus;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserDto {
    pub name: String,                 // Name of the user
    pub email: String,                // Email address
    pub phone_number: String,         // Phone number
    pub password: String,             // Password
    pub title: Option<String>,        // Optional title
    pub role_id: Option<Uuid>,        // Optional role ID
    pub school_id: Option<Uuid>,      // Optional associated school ID
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserDto {
    pub name: Option<String>,               // Optional updated name of the user
    pub email: Option<String>,              // Optional updated email address
    pub phone_number: Option<String>,       // Optional updated phone number
    pub password: Option<String>,           // Optional updated password
    pub title: Option<String>,              // Optional updated title
    pub status: Option<UserStatus>,              // Optional updated title
    pub role_id: Option<Uuid>,              // Optional updated role ID
    pub school_id: Option<Uuid>,            // Optional updated school ID
}
