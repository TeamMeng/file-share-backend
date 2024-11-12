mod files;
mod shared_links;
mod users;

use serde::{Deserialize, Serialize};
use validator::Validate;

pub use files::*;
pub use shared_links::*;
pub use users::*;

#[derive(Validate, Serialize, Deserialize)]
pub struct SearchQueryByEmail {
    #[validate(length(min = 1, message = "Query is required"))]
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterEmail {
    pub email: String,
}

impl FilterEmail {
    pub fn new(user: &Users) -> Self {
        Self {
            email: user.email.to_owned(),
        }
    }

    pub fn filter_emails(user: &[Users]) -> Vec<Self> {
        user.iter().map(Self::new).collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailListResponse {
    pub status: String,
    pub email: Vec<FilterEmail>,
}
