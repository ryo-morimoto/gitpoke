use crate::domain::badge::{Badge, BadgeStyle};
use crate::domain::user::UserId;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait BadgeRepository: Send + Sync {
    async fn get_poke_count(&self, user_id: &UserId) -> Result<u64, Box<dyn Error>>;
    
    async fn generate_badge(
        &self,
        user_id: &UserId,
        style: BadgeStyle
    ) -> Result<Badge, Box<dyn Error>>;
}

pub struct PostgresBadgeRepository {
    // TODO: Add database pool
}

impl PostgresBadgeRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl BadgeRepository for PostgresBadgeRepository {
    async fn get_poke_count(&self, _user_id: &UserId) -> Result<u64, Box<dyn Error>> {
        todo!()
    }
    
    async fn generate_badge(
        &self,
        _user_id: &UserId,
        _style: BadgeStyle
    ) -> Result<Badge, Box<dyn Error>> {
        todo!()
    }
}