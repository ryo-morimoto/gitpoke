use crate::domain::badge::{Badge, BadgeStyle};
use crate::domain::user::UserId;
use crate::infra::repositories::badge::BadgeRepository;
use std::error::Error;
use std::sync::Arc;

pub struct BadgeService {
    badge_repository: Arc<dyn BadgeRepository>,
}

impl BadgeService {
    pub fn new(badge_repository: Arc<dyn BadgeRepository>) -> Self {
        Self { badge_repository }
    }
    
    pub async fn generate_badge(
        &self,
        user_id: &UserId,
        style: BadgeStyle
    ) -> Result<Badge, Box<dyn Error>> {
        todo!()
    }
    
    pub async fn get_poke_count(&self, user_id: &UserId) -> Result<u64, Box<dyn Error>> {
        todo!()
    }
}