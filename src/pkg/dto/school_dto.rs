use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSchoolDto {
    pub name: String,                // Name of the school
    pub address: Option<String>,             // Address of the school
    pub logo_path: Option<String>,   // Optional logo path for the school
    pub subscription_id: Option<Uuid>,       // Associated subscription ID
    pub province_id: Option<String>,         // Province ID
    pub city_id: Option<String>,             // City ID
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSchoolDto {
    pub name: Option<String>,         // Optional updated name of the school
    pub address: Option<String>,      // Optional updated address
    pub logo_path: Option<String>,    // Optional updated logo path
    pub subscription_id: Option<Uuid>,// Optional updated subscription ID
    pub province_id: Option<String>,  // Optional updated province ID
    pub city_id: Option<String>,      // Optional updated city ID
}
