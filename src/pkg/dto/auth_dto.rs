use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterDto {
    pub name: String,                 // Name of the user
    pub email: String,                // Email address
    pub phone_number: String,         // Phone number
    pub password: String,             // Password
    pub school_name: String,
}
