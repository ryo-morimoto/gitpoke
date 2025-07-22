//! 共通バリデーション機能
//! 
//! このファイルは以下を定義：
//! - バリデーショントレイト
//! - 共通バリデーションエラー
//! - バリデーションヘルパー関数

use thiserror::Error;

/// バリデーションエラー
/// 
/// ドメインモデルの検証で発生するエラー
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// 無効なユーザー名
    #[error("無効なユーザー名: {reason}")]
    InvalidUsername { reason: String },
    
    /// 無効な長さ
    #[error("無効な長さ: {field}は{min}〜{max}文字である必要があります")]
    InvalidLength { 
        field: String,
        min: usize,
        max: usize,
    },
    
    /// 無効な文字
    #[error("無効な文字: {field}に使用できない文字が含まれています")]
    InvalidCharacters { field: String },
    
    /// 無効な形式
    #[error("無効な形式: {field}の形式が正しくありません")]
    InvalidFormat { field: String },
    
    /// 必須フィールドが空
    #[error("必須フィールドが空です: {field}")]
    Required { field: String },
}

/// バリデーション可能な型のトレイト
/// 
/// Parse, Don't Validateパターンを実装
pub trait Validated: Sized {
    /// バリデーションエラーの型
    type Error;
    
    /// 自身を検証し、有効な場合は自身を返す
    /// 
    /// # Returns
    /// * `Ok(self)` - 検証成功
    /// * `Err(error)` - 検証失敗
    fn validate(self) -> Result<Self, Self::Error>;
}

/// 文字列長のバリデーション
/// 
/// # Arguments
/// * `value` - 検証する文字列
/// * `field_name` - フィールド名（エラーメッセージ用）
/// * `min` - 最小長
/// * `max` - 最大長
/// 
/// # Returns
/// * `Ok(())` - 有効な長さ
/// * `Err(ValidationError)` - 無効な長さ
pub fn validate_length(
    value: &str,
    field_name: &str,
    min: usize,
    max: usize,
) -> Result<(), ValidationError> {
    let len = value.len();
    if len < min || len > max {
        return Err(ValidationError::InvalidLength {
            field: field_name.to_string(),
            min,
            max,
        });
    }
    Ok(())
}

/// 必須フィールドのバリデーション
/// 
/// # Arguments
/// * `value` - 検証する文字列
/// * `field_name` - フィールド名（エラーメッセージ用）
/// 
/// # Returns
/// * `Ok(())` - 空でない
/// * `Err(ValidationError)` - 空文字列
pub fn validate_required(value: &str, field_name: &str) -> Result<(), ValidationError> {
    if value.is_empty() {
        return Err(ValidationError::Required {
            field: field_name.to_string(),
        });
    }
    Ok(())
}

/// ASCII英数字とハイフンのバリデーション
/// 
/// GitHubユーザー名などで使用
/// 
/// # Arguments
/// * `value` - 検証する文字列
/// * `field_name` - フィールド名（エラーメッセージ用）
/// * `allow_hyphen` - ハイフンを許可するか
/// 
/// # Returns
/// * `Ok(())` - 有効な文字のみ
/// * `Err(ValidationError)` - 無効な文字を含む
pub fn validate_alphanumeric(
    value: &str,
    field_name: &str,
    allow_hyphen: bool,
) -> Result<(), ValidationError> {
    let is_valid = value.chars().all(|c| {
        c.is_ascii_alphanumeric() || (allow_hyphen && c == '-')
    });
    
    if !is_valid {
        return Err(ValidationError::InvalidCharacters {
            field: field_name.to_string(),
        });
    }
    Ok(())
}

/// GitHubユーザー名の形式バリデーション
/// 
/// - 英数字とハイフンのみ
/// - ハイフンで始まらない・終わらない
/// - 連続するハイフンは不可
/// 
/// # Arguments
/// * `username` - 検証するユーザー名
/// 
/// # Returns
/// * `Ok(())` - 有効な形式
/// * `Err(ValidationError)` - 無効な形式
pub fn validate_github_username_format(username: &str) -> Result<(), ValidationError> {
    // 空チェック
    validate_required(username, "username")?;
    
    // 長さチェック（1〜39文字）
    validate_length(username, "username", 1, 39)?;
    
    // 文字チェック
    validate_alphanumeric(username, "username", true)?;
    
    // ハイフンの位置チェック
    if username.starts_with('-') || username.ends_with('-') {
        return Err(ValidationError::InvalidFormat {
            field: "username".to_string(),
        });
    }
    
    // 連続するハイフンのチェック
    if username.contains("--") {
        return Err(ValidationError::InvalidFormat {
            field: "username".to_string(),
        });
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_length() {
        // 正常系
        assert!(validate_length("hello", "field", 1, 10).is_ok());
        assert!(validate_length("a", "field", 1, 10).is_ok());
        assert!(validate_length("1234567890", "field", 1, 10).is_ok());
        
        // 異常系
        assert!(validate_length("", "field", 1, 10).is_err());
        assert!(validate_length("12345678901", "field", 1, 10).is_err());
        
        // エラーメッセージの確認
        match validate_length("", "field", 1, 10) {
            Err(ValidationError::InvalidLength { field, min, max }) => {
                assert_eq!(field, "field");
                assert_eq!(min, 1);
                assert_eq!(max, 10);
            }
            _ => panic!("Expected InvalidLength error"),
        }
    }
    
    #[test]
    fn test_validate_required() {
        // 正常系
        assert!(validate_required("value", "field").is_ok());
        assert!(validate_required(" ", "field").is_ok()); // 空白文字はOK
        
        // 異常系
        assert!(validate_required("", "field").is_err());
        
        // エラーメッセージの確認
        match validate_required("", "field") {
            Err(ValidationError::Required { field }) => {
                assert_eq!(field, "field");
            }
            _ => panic!("Expected Required error"),
        }
    }
    
    #[test]
    fn test_validate_alphanumeric() {
        // ハイフンを許可する場合
        assert!(validate_alphanumeric("abc123", "field", true).is_ok());
        assert!(validate_alphanumeric("test-user", "field", true).is_ok());
        assert!(validate_alphanumeric("ABC", "field", true).is_ok());
        assert!(validate_alphanumeric("123", "field", true).is_ok());
        
        // ハイフンを許可しない場合
        assert!(validate_alphanumeric("abc123", "field", false).is_ok());
        assert!(validate_alphanumeric("test-user", "field", false).is_err());
        
        // 無効な文字
        assert!(validate_alphanumeric("test user", "field", true).is_err());
        assert!(validate_alphanumeric("test@user", "field", true).is_err());
        assert!(validate_alphanumeric("テスト", "field", true).is_err());
        
        // エラーメッセージの確認
        match validate_alphanumeric("test@user", "field", true) {
            Err(ValidationError::InvalidCharacters { field }) => {
                assert_eq!(field, "field");
            }
            _ => panic!("Expected InvalidCharacters error"),
        }
    }
    
    #[test]
    fn test_validate_github_username_format() {
        // 有効なユーザー名
        assert!(validate_github_username_format("octocat").is_ok());
        assert!(validate_github_username_format("user-123").is_ok());
        assert!(validate_github_username_format("a").is_ok());
        assert!(validate_github_username_format("Test-User-123").is_ok());
        
        // 無効なユーザー名
        assert!(validate_github_username_format("").is_err());
        assert!(validate_github_username_format("-user").is_err());
        assert!(validate_github_username_format("user-").is_err());
        assert!(validate_github_username_format("user--name").is_err());
        assert!(validate_github_username_format("user name").is_err());
        assert!(validate_github_username_format("user.name").is_err());
        assert!(validate_github_username_format("user@name").is_err());
        assert!(validate_github_username_format("a".repeat(40).as_str()).is_err());
    }
    
    #[test]
    fn test_validation_error_display() {
        // エラーメッセージの表示確認
        let err = ValidationError::InvalidUsername { reason: "test reason".to_string() };
        assert_eq!(err.to_string(), "無効なユーザー名: test reason");
        
        let err = ValidationError::InvalidLength { 
            field: "username".to_string(), 
            min: 1, 
            max: 39 
        };
        assert_eq!(err.to_string(), "無効な長さ: usernameは1〜39文字である必要があります");
        
        let err = ValidationError::InvalidCharacters { field: "username".to_string() };
        assert_eq!(err.to_string(), "無効な文字: usernameに使用できない文字が含まれています");
        
        let err = ValidationError::InvalidFormat { field: "username".to_string() };
        assert_eq!(err.to_string(), "無効な形式: usernameの形式が正しくありません");
        
        let err = ValidationError::Required { field: "username".to_string() };
        assert_eq!(err.to_string(), "必須フィールドが空です: username");
    }
}