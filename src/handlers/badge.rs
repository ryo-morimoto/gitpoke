//! バッジ生成ハンドラー
//! 
//! このファイルは以下を定義：
//! - バッジ生成エンドポイント
//! - キャッシュ戦略の実装
//! - SVGレスポンスの構築

use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Response},
    http::{header, StatusCode},
};
use serde::{Deserialize, Serialize};

use crate::app::dependencies::AppDependencies;
use crate::domain::badge::{BadgeState, BadgeSvg};
use crate::domain::user::Username;
use crate::error::{AppResult, HandlerError};
use crate::use_cases::generate_badge as use_case;

/// バッジリクエストのクエリパラメータ
#[derive(Debug, Deserialize)]
pub struct BadgeQuery {
    /// バッジスタイル（将来実装）
    /// 例: flat, flat-square, plastic
    #[serde(default)]
    pub style: Option<String>,
    
    /// キャッシュ無効化パラメータ
    /// 例: ?cache_bust=1234567890
    #[serde(default)]
    pub cache_bust: Option<String>,
    
    /// インタラクティブモード
    /// true の場合、クリック可能なバッジを生成
    #[serde(default)]
    pub interactive: Option<bool>,
}

/// バッジ生成エンドポイント
/// 
/// GET /badge/:username.svg
/// 
/// GitHubユーザーのアクティビティ状態を示すSVGバッジを生成
/// 
/// # Arguments
/// * `username` - GitHubユーザー名
/// * `query` - クエリパラメータ
/// * `deps` - アプリケーション依存性
/// 
/// # Returns
/// * 200 OK - SVGバッジ
/// * 404 Not Found - ユーザーが見つからない
/// * 500 Internal Server Error - 生成エラー
pub async fn generate_badge(
    Path(username): Path<String>,
    Query(query): Query<BadgeQuery>,
    State(deps): State<AppDependencies>,
) -> AppResult<Response> {
    // ユーザー名のバリデーション
    let username = Username::parse(username)
        .map_err(|_| HandlerError::BadRequest("Invalid username format".to_string()))?;
    
    // キャッシュキーの生成
    let cache_key = format!("badge:{}:v1", username.as_str());
    
    // キャッシュからの取得を試みる
    if let Some(cached_svg) = get_cached_badge(&deps, &cache_key).await? {
        return Ok(build_svg_response(cached_svg, true));
    }
    
    // バッジ生成のユースケースを実行
    let result = use_case::execute(
        &username,
        &deps,
        query.interactive.unwrap_or(false),
    ).await?;
    
    // キャッシュに保存
    save_badge_to_cache(&deps, &cache_key, &result.badge).await?;
    
    // レスポンスを構築
    Ok(build_svg_response(result.badge, false))
}

/// キャッシュからバッジを取得
/// 
/// # Arguments
/// * `deps` - アプリケーション依存性
/// * `cache_key` - キャッシュキー
/// 
/// # Returns
/// * `Some(BadgeSvg)` - キャッシュヒット
/// * `None` - キャッシュミス
async fn get_cached_badge(
    deps: &AppDependencies,
    cache_key: &str,
) -> AppResult<Option<BadgeSvg>> {
    // Redisからの取得を試みる
    if let Some(cached_content) = deps.cache_service.get(cache_key).await? {
        // TODO: BadgeSvgのデシリアライズ
        // - JSON形式で保存されているものを復元
        return Ok(Some(BadgeSvg {
            content: cached_content,
            cache_ttl: 300, // デフォルトTTL
            is_interactive: false,
        }));
    }
    
    // Cloud Storageからの取得を試みる（コールドキャッシュ）
    // TODO: 実装
    // - storage_service.get_badge(username) を実行
    // - 存在すれば、Redisにも保存してから返す
    
    Ok(None)
}

/// バッジをキャッシュに保存
/// 
/// # Arguments
/// * `deps` - アプリケーション依存性
/// * `cache_key` - キャッシュキー
/// * `badge` - 保存するバッジ
async fn save_badge_to_cache(
    deps: &AppDependencies,
    cache_key: &str,
    badge: &BadgeSvg,
) -> AppResult<()> {
    // Redisに保存
    deps.cache_service.set(
        cache_key,
        &badge.content,
        badge.cache_ttl,
    ).await?;
    
    // Cloud Storageにも非同期で保存（エラーは無視）
    // TODO: 実装
    // - tokio::spawn で非同期実行
    // - storage_service.save_badge(username, badge) を実行
    
    Ok(())
}

/// SVGレスポンスを構築
/// 
/// # Arguments
/// * `badge` - バッジデータ
/// * `from_cache` - キャッシュから取得したかどうか
/// 
/// # Returns
/// * `Response` - HTTPレスポンス
fn build_svg_response(badge: BadgeSvg, from_cache: bool) -> Response {
    let mut response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, badge.content_type())
        .header(header::CACHE_CONTROL, badge.cache_control())
        .header("X-Content-Type-Options", "nosniff");
    
    // キャッシュヒットの場合はヘッダーを追加
    if from_cache {
        response = response.header("X-Cache", "HIT");
    } else {
        response = response.header("X-Cache", "MISS");
    }
    
    // CORS対応（GitHub.comからのアクセスを許可）
    response = response
        .header("Access-Control-Allow-Origin", "https://github.com")
        .header("Access-Control-Allow-Methods", "GET");
    
    // インタラクティブバッジの場合は追加ヘッダー
    if badge.is_interactive {
        response = response
            .header("Access-Control-Allow-Credentials", "true");
    }
    
    response
        .body(badge.content.into())
        .unwrap()
        .into_response()
}

/// バッジプレビューエンドポイント（開発用）
/// 
/// GET /api/badge/preview
/// 
/// すべてのバッジスタイルをプレビュー
#[allow(dead_code)]
pub async fn preview_badges() -> impl IntoResponse {
    // TODO: 実装
    // - 各種状態のバッジを生成
    // - HTMLで一覧表示
    StatusCode::NOT_IMPLEMENTED
}