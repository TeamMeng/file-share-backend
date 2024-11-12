use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct SharedLinks {
    id: Uuid,
    pub file_id: Option<Uuid>,
    pub recipient_user_id: Option<Uuid>,
    pub password: String,
    pub expiration_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
