use crate::domain::poke::{Poke, PokeId};
use crate::domain::user::UserId;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait PokeRepository: Send + Sync {
    async fn find_by_id(&self, id: &PokeId) -> Result<Option<Poke>, Box<dyn Error>>;
    
    async fn find_by_sender_and_recipient(
        &self,
        sender_id: &UserId,
        recipient_id: &UserId
    ) -> Result<Vec<Poke>, Box<dyn Error>>;
    
    async fn save(&self, poke: &Poke) -> Result<(), Box<dyn Error>>;
    
    async fn count_by_recipient(&self, recipient_id: &UserId) -> Result<u64, Box<dyn Error>>;
    
    async fn list_recent(
        &self,
        limit: usize,
        offset: usize
    ) -> Result<Vec<Poke>, Box<dyn Error>>;
}

pub struct PostgresPokeRepository {
    // TODO: Add database pool
}

impl PostgresPokeRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl PokeRepository for PostgresPokeRepository {
    async fn find_by_id(&self, _id: &PokeId) -> Result<Option<Poke>, Box<dyn Error>> {
        todo!()
    }
    
    async fn find_by_sender_and_recipient(
        &self,
        _sender_id: &UserId,
        _recipient_id: &UserId
    ) -> Result<Vec<Poke>, Box<dyn Error>> {
        todo!()
    }
    
    async fn save(&self, _poke: &Poke) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    
    async fn count_by_recipient(&self, _recipient_id: &UserId) -> Result<u64, Box<dyn Error>> {
        todo!()
    }
    
    async fn list_recent(
        &self,
        _limit: usize,
        _offset: usize
    ) -> Result<Vec<Poke>, Box<dyn Error>> {
        todo!()
    }
}