use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, MultipartForm)]
pub struct CreateSchoolDto {
    pub name: Text<String>,                // Name of the school
    pub address: Option<Text<String>>,             // Address of the school
    // pub logo_path: Option<String>,   // Optional logo path for the school
    pub subscription_id: Option<Text<Uuid>>,       // Associated subscription ID
    pub province_id: Option<Text<String>>,         // Province ID
    pub city_id: Option<Text<String>>,             // City ID
    pub logo: Option<TempFile>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSchoolDto {
    pub name: Option<String>,         // Optional updated name of the school
    pub address: Option<String>,      // Optional updated address
    pub logo_path: Option<String>,    // Optional updated logo path
    pub subscription_id: Option<Uuid>, // Optional updated subscription ID
    pub province_id: Option<String>,  // Optional updated province ID
    pub city_id: Option<String>,      // Optional updated city ID
}
