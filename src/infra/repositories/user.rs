use crate::domain::user::{User, UserId};
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, Box<dyn Error>>;
    
    async fn find_by_github_id(&self, github_id: u64) -> Result<Option<User>, Box<dyn Error>>;
    
    async fn save(&self, user: &User) -> Result<(), Box<dyn Error>>;
    
    async fn update(&self, user: &User) -> Result<(), Box<dyn Error>>;
    
    async fn delete(&self, id: &UserId) -> Result<(), Box<dyn Error>>;
}

pub struct PostgresUserRepository {
    // TODO: Add database pool
}

impl PostgresUserRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, _id: &UserId) -> Result<Option<User>, Box<dyn Error>> {
        todo!()
    }
    
    async fn find_by_github_id(&self, _github_id: u64) -> Result<Option<User>, Box<dyn Error>> {
        todo!()
    }
    
    async fn save(&self, _user: &User) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    
    async fn update(&self, _user: &User) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    
    async fn delete(&self, _id: &UserId) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}