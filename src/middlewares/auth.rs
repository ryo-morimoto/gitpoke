//! <ßÉë¦§¢
//! 
//! SnÕ¡¤ëoå’š©
//! - <LÅj¨óÉİ¤óÈ(nßÉë¦§¢
//! - ª×·çÊë<nßÉë¦§¢
//! - »Ã·çó¡

use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode, HeaderMap},
    RequestPartsExt,
};
use axum_extra::extract::CookieJar;

use crate::app::dependencies::AppDependencies;
use crate::domain::user::{Username, RegisteredUser};
use crate::error::HandlerError;

/// <æü¶ü
/// 
/// FromRequestParts’ŸÅWÏóÉéügô¥Ö—ïı
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub username: Username,
    pub session_id: String,
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = HandlerError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // CookieK‰»Ã·çó’Ö—
        let cookies = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| HandlerError::Unauthorized)?;
        
        let session_id = cookies
            .get("gitpoke_session")
            .map(|c| c.value().to_string())
            .ok_or(HandlerError::Unauthorized)?;
        
        // AppDependencies’Ö—
        let State(deps) = State::<AppDependencies>::from_request_parts(parts, state)
            .await
            .map_err(|_| HandlerError::InternalServerError("Failed to get dependencies".to_string()))?;
        
        // RedisK‰»Ã·çóÅ1’Ö—
        let session_key = format!("session:{}", session_id);
        let session_data = deps.cache_service
            .get(&session_key)
            .await
            .map_err(|_| HandlerError::InternalServerError("Session lookup failed".to_string()))?
            .ok_or(HandlerError::Unauthorized)?;
        
        // »Ã·çóÇü¿’Ñü¹
        let session: serde_json::Value = serde_json::from_str(&session_data)
            .map_err(|_| HandlerError::InternalServerError("Invalid session data".to_string()))?;
        
        let username = session["username"]
            .as_str()
            .ok_or(HandlerError::Unauthorized)?;
        
        Ok(AuthenticatedUser {
            username: Username::new(username.to_string())
                .map_err(|_| HandlerError::Unauthorized)?,
            session_id,
        })
    }
}

/// ª×·çÊë<æü¶ü
/// 
/// <oÅgojDL<n4oæü¶üÅ1’Ö—
#[derive(Debug, Clone)]
pub struct OptionalUser(pub Option<AuthenticatedUser>);

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for OptionalUser
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // AuthenticatedUsernÖ—’f‹
        let user = AuthenticatedUser::from_request_parts(parts, state).await.ok();
        Ok(OptionalUser(user))
    }
}

/// <LÅjÏóÉéü(nØëÑü¢p
/// 
/// Sn¢poŒ¹’Û'n_k‹UŒfD~Y
/// °WD³üÉgoAuthenticatedUser extractor’ô¥(WfO`UD
#[deprecated(note = "Use AuthenticatedUser extractor directly in handlers")]
pub async fn require_auth(
    parts: &mut Parts,
    state: &AppDependencies,
) -> Result<Username, HandlerError> {
    let user = AuthenticatedUser::from_request_parts(parts, state).await?;
    Ok(user.username)
}

/// ª×·çÊë<(nØëÑü¢p
/// 
/// Sn¢poŒ¹’Û'n_k‹UŒfD~Y
/// °WD³üÉgoOptionalUser extractor’ô¥(WfO`UD
#[deprecated(note = "Use OptionalUser extractor directly in handlers")]
pub async fn optional_auth(
    parts: &mut Parts,
    state: &AppDependencies,
) -> Option<Username> {
    let user = OptionalUser::from_request_parts(parts, state).await.ok()?;
    user.0.map(|u| u.username)
}