//! ハンドラー共通ユーティリティ
//! 
//! このファイルは以下を定義：
//! - リクエストからの情報抽出
//! - レスポンスの構築ヘルパー
//! - 共通バリデーション

use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::net::IpAddr;

/// クライアントIPアドレスを取得
/// 
/// 以下の順番で確認：
/// 1. X-Forwarded-For ヘッダー（Cloud Run/プロキシ環境）
/// 2. X-Real-IP ヘッダー
/// 3. RemoteAddr（直接接続）
/// 
/// # Arguments
/// * `headers` - HTTPヘッダー
/// 
/// # Returns
/// * IPアドレス文字列（取得できない場合は "unknown"）
pub fn get_client_ip(headers: &HeaderMap) -> String {
    // X-Forwarded-For をチェック（カンマ区切りの場合は最初のIP）
    if let Some(forwarded) = headers.get("x-forwarded-for") {
        if let Ok(value) = forwarded.to_str() {
            if let Some(ip) = value.split(',').next() {
                return ip.trim().to_string();
            }
        }
    }
    
    // X-Real-IP をチェック
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(value) = real_ip.to_str() {
            return value.to_string();
        }
    }
    
    // TODO: RemoteAddrから取得（axumのExtractorを使用）
    
    "unknown".to_string()
}

/// User-Agentを取得
/// 
/// # Arguments
/// * `headers` - HTTPヘッダー
/// 
/// # Returns
/// * User-Agent文字列（存在しない場合は None）
pub fn get_user_agent(headers: &HeaderMap) -> Option<String> {
    headers
        .get("user-agent")
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string())
}

/// 成功レスポンスを構築
/// 
/// 統一されたJSON形式の成功レスポンスを生成
/// 
/// # Arguments
/// * `data` - レスポンスデータ
/// 
/// # Example
/// ```
/// let response = success_response(json!({
///     "message": "Operation completed successfully"
/// }));
/// ```
pub fn success_response<T: Serialize>(data: T) -> Response {
    (StatusCode::OK, Json(SuccessResponse { data })).into_response()
}

/// 成功レスポンスの構造
#[derive(Debug, Serialize)]
struct SuccessResponse<T> {
    pub data: T,
}

/// エラーレスポンスを構築
/// 
/// 統一されたJSON形式のエラーレスポンスを生成
/// 
/// # Arguments
/// * `status` - HTTPステータスコード
/// * `message` - エラーメッセージ
/// * `details` - 詳細情報（オプション）
pub fn error_response(
    status: StatusCode,
    message: &str,
    details: Option<serde_json::Value>,
) -> Response {
    let error = ErrorResponse {
        error: ErrorDetail {
            code: status.as_u16(),
            message: message.to_string(),
            details,
        },
    };
    
    (status, Json(error)).into_response()
}

/// エラーレスポンスの構造
#[derive(Debug, Serialize)]
struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Serialize)]
struct ErrorDetail {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}


/// リクエストIDを生成
/// 
/// トレーシング用の一意なリクエストIDを生成
pub fn generate_request_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// CORSプリフライトチェック
/// 
/// OPTIONSメソッドのリクエストに対する処理
pub fn handle_preflight() -> Response {
    Response::builder()
        .status(StatusCode::NO_CONTENT)
        .header("Access-Control-Allow-Origin", "https://github.com")
        .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type, Authorization")
        .header("Access-Control-Max-Age", "86400")
        .body(Default::default())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_client_ip() {
        let mut headers = HeaderMap::new();
        
        // X-Forwarded-For
        headers.insert("x-forwarded-for", "192.168.1.1, 10.0.0.1".parse().unwrap());
        assert_eq!(get_client_ip(&headers), "192.168.1.1");
        
        // X-Real-IP
        headers.clear();
        headers.insert("x-real-ip", "192.168.1.2".parse().unwrap());
        assert_eq!(get_client_ip(&headers), "192.168.1.2");
        
        // No headers
        headers.clear();
        assert_eq!(get_client_ip(&headers), "unknown");
    }
    
}