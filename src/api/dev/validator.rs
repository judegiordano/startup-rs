use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct CreatePing {
    #[validate(length(
        min = 1,
        max = 20,
        message = "body.message must be between 1 and 20 characters"
    ))]
    pub message: String,
}
