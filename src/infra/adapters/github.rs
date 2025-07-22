use async_trait::async_trait;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct GitHubUser {
    pub id: u64,
    pub login: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: String,
}

#[async_trait]
pub trait GitHubAdapter: Send + Sync {
    async fn get_user(&self, access_token: &str) -> Result<GitHubUser, Box<dyn Error>>;
    
    async fn get_user_by_username(&self, username: &str) -> Result<GitHubUser, Box<dyn Error>>;
    
    async fn verify_token(&self, access_token: &str) -> Result<bool, Box<dyn Error>>;
}

pub struct GitHubApiAdapter {
    client: reqwest::Client,
}

impl GitHubApiAdapter {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl GitHubAdapter for GitHubApiAdapter {
    async fn get_user(&self, _access_token: &str) -> Result<GitHubUser, Box<dyn Error>> {
        todo!()
    }
    
    async fn get_user_by_username(&self, _username: &str) -> Result<GitHubUser, Box<dyn Error>> {
        todo!()
    }
    
    async fn verify_token(&self, _access_token: &str) -> Result<bool, Box<dyn Error>> {
        todo!()
    }
}