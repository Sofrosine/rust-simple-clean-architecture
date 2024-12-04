use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateRoleDto {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoleDto {
    pub name: Option<String>,
}
