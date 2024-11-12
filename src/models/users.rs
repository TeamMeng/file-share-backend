use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::{ReceiveFile, SentFiles};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct Users {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub public_key: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Validate, Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least characters"))]
    pub password: String,

    #[validate(
        length(min = 6, message = "Password confirm must be at least characters"),
        must_match(other = "password", message = "Password not match")
    )]
    #[serde(rename = "passwordConfirm")]
    pub password_confirm: String,
}

#[derive(Validate, Debug, Serialize, Deserialize)]
pub struct LoginUser {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub public_key: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSendFile {
    pub file_id: String,
    pub file_name: String,
    pub recipient_email: String,
    pub expiration_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSendFileListResponse {
    pub status: String,
    pub files: Vec<UserSendFile>,
    pub results: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserReceiveFile {
    pub file_id: String,
    pub file_name: String,
    pub sender_email: String,
    pub expiration_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserReceiveFileListResponse {
    pub status: String,
    pub files: Vec<UserReceiveFile>,
    pub results: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponse {
    pub status: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

#[derive(Validate, Debug, Serialize, Deserialize)]
pub struct UserNameUpdate {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Validate, Debug, Serialize, Deserialize)]
pub struct UserPasswordUpdate {
    #[validate(length(min = 6, message = "New password is must be at least 6 characters"))]
    pub new_password: String,

    #[validate(
        length(
            min = 6,
            message = "New password confirm is must be at least 6 characters"
        ),
        must_match(other = "new_password", message = "New password do not match")
    )]
    pub new_password_confirm: String,

    #[validate(length(min = 6, message = "Old password must be at least 6 characters"))]
    pub old_password: String,
}

impl FilterUser {
    pub fn new(user: &Users) -> Self {
        Self {
            id: user.id.to_string(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            public_key: user.public_key.to_owned(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl UserSendFile {
    pub fn filter_send_user_file(file_data: &SentFiles) -> Self {
        Self {
            file_id: file_data.file_id.to_string(),
            file_name: file_data.file_name.to_owned(),
            recipient_email: file_data.recipient_email.to_owned(),
            expiration_date: file_data.expiration_date,
            created_at: file_data.created_at,
        }
    }

    pub fn filter_send_user_files(user: &[SentFiles]) -> Vec<Self> {
        user.iter().map(Self::filter_send_user_file).collect()
    }
}

impl UserReceiveFile {
    pub fn filter_receive_user_file(file_data: &ReceiveFile) -> Self {
        Self {
            file_id: file_data.file_id.to_string(),
            file_name: file_data.file_name.to_owned(),
            sender_email: file_data.sender_email.to_owned(),
            expiration_date: file_data.expiration_date,
            created_at: file_data.created_at,
        }
    }

    pub fn filter_receive_user_files(user: &[ReceiveFile]) -> Vec<Self> {
        user.iter().map(Self::filter_receive_user_file).collect()
    }
}
