use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait DatabaseAdapter: Send + Sync {
    async fn execute(&self, query: &str, params: &[&dyn rusqlite::ToSql]) -> Result<usize, Box<dyn Error>>;
    
    async fn query_one<T>(&self, query: &str, params: &[&dyn rusqlite::ToSql], mapper: fn(&rusqlite::Row) -> Result<T, rusqlite::Error>) -> Result<Option<T>, Box<dyn Error>>;
    
    async fn query_many<T>(&self, query: &str, params: &[&dyn rusqlite::ToSql], mapper: fn(&rusqlite::Row) -> Result<T, rusqlite::Error>) -> Result<Vec<T>, Box<dyn Error>>;
    
    async fn transaction<F, R>(&self, f: F) -> Result<R, Box<dyn Error>>
    where
        F: FnOnce() -> Result<R, Box<dyn Error>> + Send,
        R: Send;
}

pub struct PostgresAdapter {
    // TODO: Add connection pool
}

impl PostgresAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl DatabaseAdapter for PostgresAdapter {
    async fn execute(&self, _query: &str, _params: &[&dyn rusqlite::ToSql]) -> Result<usize, Box<dyn Error>> {
        todo!()
    }
    
    async fn query_one<T>(&self, _query: &str, _params: &[&dyn rusqlite::ToSql], _mapper: fn(&rusqlite::Row) -> Result<T, rusqlite::Error>) -> Result<Option<T>, Box<dyn Error>> {
        todo!()
    }
    
    async fn query_many<T>(&self, _query: &str, _params: &[&dyn rusqlite::ToSql], _mapper: fn(&rusqlite::Row) -> Result<T, rusqlite::Error>) -> Result<Vec<T>, Box<dyn Error>> {
        todo!()
    }
    
    async fn transaction<F, R>(&self, _f: F) -> Result<R, Box<dyn Error>>
    where
        F: FnOnce() -> Result<R, Box<dyn Error>> + Send,
        R: Send,
    {
        todo!()
    }
}