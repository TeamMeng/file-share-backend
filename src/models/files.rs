use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct Files {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub file_name: String,
    pub file_size: i64,
    pub encrypted_aes_key: Vec<u8>,
    pub encrypted_file: Vec<u8>,
    pub iv: Vec<u8>,
    pub created_ad: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SentFiles {
    pub file_id: Uuid,
    pub file_name: String,
    pub recipient_email: String,
    pub expiration_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
pub struct ReceiveFile {
    pub file_id: Uuid,
    pub file_name: String,
    pub sender_email: String,
    pub expiration_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Validate, Debug, Serialize, Deserialize)]
pub struct FileUpload {
    #[validate(email(message = "Invalid email format"))]
    pub recipient_email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,

    #[validate(custom(function = "validate_expiration_date"))]
    pub expiration_date: String,
}

fn validate_expiration_date(expiration_date: &str) -> Result<(), ValidationError> {
    if expiration_date.is_empty() {
        let mut error = ValidationError::new("expiration_date_required");
        error.message = Some("Expiration date is required.".into());
        return Err(error);
    }

    let parsed_date = DateTime::parse_from_rfc3339(expiration_date).map_err(|_| {
        let mut error = ValidationError::new("invalid_date_format");
        error.message =
            Some("Invalid date format. Expected format is YYYY-MM-DDTHH:MM:SS.ssssssZ.".into());
        error
    })?;

    let now = Utc::now();

    if parsed_date <= now {
        let mut error = ValidationError::new("expiration_date_future");
        error.message = Some("Expiration date must be in the future.".into());
        return Err(error);
    }

    Ok(())
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct RetrieveFile {
    #[validate(length(min = 1, message = "Shared id is required"))]
    pub shared_id: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}
