use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub data: T,        // Dynamic data type
    pub page: u32,      // Current page
    pub page_size: u32,  // Items per page
    pub total_pages: u32,
    pub total_data: u32,
}

#[derive(Serialize)]
pub struct Response<T> {
    pub data: T,        // Dynamic data type
}