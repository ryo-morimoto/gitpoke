use crate::domain::user::{User, UserId};
use crate::infra::repositories::user::UserRepository;
use std::error::Error;
use std::sync::Arc;

pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
    
    pub async fn get_user(&self, id: &UserId) -> Result<Option<User>, Box<dyn Error>> {
        todo!()
    }
    
    pub async fn get_user_by_github_id(&self, github_id: u64) -> Result<Option<User>, Box<dyn Error>> {
        todo!()
    }
    
    pub async fn create_user(&self, user: User) -> Result<User, Box<dyn Error>> {
        todo!()
    }
    
    pub async fn update_user(&self, user: User) -> Result<User, Box<dyn Error>> {
        todo!()
    }
    
    pub async fn delete_user(&self, id: &UserId) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}