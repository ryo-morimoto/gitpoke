use crate::domain::poke::{Poke, PokeId};
use crate::domain::user::UserId;
use crate::infra::repositories::poke::PokeRepository;
use std::error::Error;
use std::sync::Arc;

pub struct PokeService {
    poke_repository: Arc<dyn PokeRepository>,
}

impl PokeService {
    pub fn new(poke_repository: Arc<dyn PokeRepository>) -> Self {
        Self { poke_repository }
    }
    
    pub async fn send_poke(
        &self,
        sender_id: UserId,
        recipient_id: UserId
    ) -> Result<Poke, Box<dyn Error>> {
        todo!()
    }
    
    pub async fn get_poke(&self, id: &PokeId) -> Result<Option<Poke>, Box<dyn Error>> {
        todo!()
    }
    
    pub async fn get_pokes_between(
        &self,
        sender_id: &UserId,
        recipient_id: &UserId
    ) -> Result<Vec<Poke>, Box<dyn Error>> {
        todo!()
    }
    
    pub async fn count_received_pokes(&self, user_id: &UserId) -> Result<u64, Box<dyn Error>> {
        todo!()
    }
    
    pub async fn get_recent_pokes(
        &self,
        limit: usize,
        offset: usize
    ) -> Result<Vec<Poke>, Box<dyn Error>> {
        todo!()
    }
}