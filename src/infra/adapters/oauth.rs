use async_trait::async_trait;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct OAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub scope: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
}

#[async_trait]
pub trait OAuthAdapter: Send + Sync {
    async fn exchange_code(&self, code: &str) -> Result<OAuthToken, Box<dyn Error>>;
    
    async fn refresh_token(&self, refresh_token: &str) -> Result<OAuthToken, Box<dyn Error>>;
    
    async fn revoke_token(&self, token: &str) -> Result<(), Box<dyn Error>>;
}

pub struct GitHubOAuthAdapter {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

impl GitHubOAuthAdapter {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
        }
    }
}

#[async_trait]
impl OAuthAdapter for GitHubOAuthAdapter {
    async fn exchange_code(&self, _code: &str) -> Result<OAuthToken, Box<dyn Error>> {
        todo!()
    }
    
    async fn refresh_token(&self, _refresh_token: &str) -> Result<OAuthToken, Box<dyn Error>> {
        todo!()
    }
    
    async fn revoke_token(&self, _token: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}