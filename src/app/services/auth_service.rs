use crate::domain::user::{User, UserId};
use std::error::Error;

pub struct AuthService {
    // TODO: Add OAuth client
}

impl AuthService {
    pub fn new() -> Self {
        Self {}
    }
    
    pub async fn authenticate_github_user(
        &self,
        code: &str
    ) -> Result<User, Box<dyn Error>> {
        todo!()
    }
    
    pub async fn refresh_token(
        &self,
        refresh_token: &str
    ) -> Result<String, Box<dyn Error>> {
        todo!()
    }
    
    pub async fn validate_token(
        &self,
        token: &str
    ) -> Result<UserId, Box<dyn Error>> {
        todo!()
    }
    
    pub async fn revoke_token(
        &self,
        token: &str
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}