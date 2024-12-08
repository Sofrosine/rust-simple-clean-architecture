use base64::{DecodeError, Engine, engine::{general_purpose}};

pub fn decode_basic_auth_token(token: &str) -> Result<(String, String), DecodeError> {
    // Decode the base64-encoded string
    let decoded_bytes = general_purpose::STANDARD.decode(token.as_bytes())?;

    // Convert bytes to a string and split on the first colon
    let decoded_str = String::from_utf8_lossy(&decoded_bytes);
    let parts: Vec<&str> = decoded_str.split(':').collect();

    if parts.len() == 2 {
        Ok((parts[0].to_string(), parts[1].to_string())) // (username, password)
    } else {
        Err(DecodeError::InvalidPadding)
    }
}