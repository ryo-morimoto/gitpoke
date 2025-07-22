//! 関数型プログラミングユーティリティ
//! 
//! このファイルは以下を定義：
//! - 関数合成
//! - パイプライン処理
//! - Result/Option型の変換

use std::future::Future;

/// 2つの関数を合成
/// 
/// f: A -> B と g: B -> C から A -> C を作る
/// 
/// # Example
/// ```
/// let add_one = |x: i32| x + 1;
/// let double = |x: i32| x * 2;
/// let add_one_then_double = pipe(add_one, double);
/// assert_eq!(add_one_then_double(5), 12); // (5 + 1) * 2 = 12
/// ```
pub fn pipe<A, B, C>(
    f: impl Fn(A) -> B,
    g: impl Fn(B) -> C,
) -> impl Fn(A) -> C {
    move |a| g(f(a))
}

/// 3つの関数を合成
/// 
/// f: A -> B, g: B -> C, h: C -> D から A -> D を作る
pub fn pipe3<A, B, C, D>(
    f: impl Fn(A) -> B,
    g: impl Fn(B) -> C,
    h: impl Fn(C) -> D,
) -> impl Fn(A) -> D {
    move |a| h(g(f(a)))
}

/// Result型を返す関数の合成
/// 
/// エラーが発生した場合は早期リターン
/// 
/// # Example
/// ```
/// let parse_int = |s: &str| s.parse::<i32>();
/// let check_positive = |x: i32| -> Result<i32, &'static str> {
///     if x > 0 { Ok(x) } else { Err("not positive") }
/// };
/// let parse_positive = pipe_result(parse_int, check_positive);
/// ```
pub fn pipe_result<A, B, C, E>(
    f: impl Fn(A) -> Result<B, E>,
    g: impl Fn(B) -> Result<C, E>,
) -> impl Fn(A) -> Result<C, E> {
    move |a| f(a).and_then(|b| g(b))
}

/// Option型を返す関数の合成
/// 
/// Noneが返された場合は早期リターン
pub fn pipe_option<A, B, C>(
    f: impl Fn(A) -> Option<B>,
    g: impl Fn(B) -> Option<C>,
) -> impl Fn(A) -> Option<C> {
    move |a| f(a).and_then(|b| g(b))
}

/// 非同期関数の合成
/// 
/// 2つの非同期関数を合成
pub fn pipe_async<A, B, C, Fut1, Fut2>(
    f: impl Fn(A) -> Fut1,
    g: impl Fn(B) -> Fut2,
) -> impl Fn(A) -> impl Future<Output = C>
where
    Fut1: Future<Output = B>,
    Fut2: Future<Output = C>,
{
    move |a| async move { g(f(a).await).await }
}

/// Result<T, E>からOption<T>への変換
/// 
/// エラーの場合はNoneを返す
pub fn result_to_option<T, E>(result: Result<T, E>) -> Option<T> {
    result.ok()
}

/// Option<T>からResult<T, E>への変換
/// 
/// Noneの場合は指定されたエラーを返す
pub fn option_to_result<T, E>(option: Option<T>, error: E) -> Result<T, E> {
    option.ok_or(error)
}

/// 複数の値に対して関数を適用
/// 
/// すべて成功した場合のみOkを返す
pub fn try_map_all<T, U, E, F>(
    values: Vec<T>,
    f: F,
) -> Result<Vec<U>, E>
where
    F: Fn(T) -> Result<U, E>,
{
    values.into_iter().map(f).collect()
}

/// 条件に基づいて関数を選択
/// 
/// # Example
/// ```
/// let process = if_else(
///     |x: &i32| *x > 0,
///     |x| x * 2,
///     |x| x * -1,
/// );
/// assert_eq!(process(5), 10);
/// assert_eq!(process(-3), 3);
/// ```
pub fn if_else<T, U>(
    predicate: impl Fn(&T) -> bool,
    if_true: impl Fn(T) -> U,
    if_false: impl Fn(T) -> U,
) -> impl Fn(T) -> U {
    move |value| {
        if predicate(&value) {
            if_true(value)
        } else {
            if_false(value)
        }
    }
}

/// タプルの要素を入れ替え
pub fn swap<A, B>((a, b): (A, B)) -> (B, A) {
    (b, a)
}

/// 関数の引数の順序を入れ替え（カリー化）
pub fn flip<A, B, C>(
    f: impl Fn(A, B) -> C,
) -> impl Fn(B, A) -> C {
    move |b, a| f(a, b)
}

/// 常に同じ値を返す関数を作成
pub fn constant<T: Clone, U>(value: T) -> impl Fn(U) -> T {
    move |_| value.clone()
}

/// 恒等関数（引数をそのまま返す）
pub fn identity<T>(value: T) -> T {
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pipe() {
        let add_one = |x: i32| x + 1;
        let double = |x: i32| x * 2;
        let add_one_then_double = pipe(add_one, double);
        
        assert_eq!(add_one_then_double(5), 12);
    }
    
    #[test]
    fn test_pipe_result() {
        let parse = |s: &str| s.parse::<i32>();
        let validate = |x: i32| -> Result<i32, &'static str> {
            if x > 0 { Ok(x) } else { Err("not positive") }
        };
        let parse_positive = pipe_result(parse, validate);
        
        assert_eq!(parse_positive("42"), Ok(42));
        assert!(parse_positive("-5").is_err());
        assert!(parse_positive("abc").is_err());
    }
    
    #[test]
    fn test_try_map_all() {
        let values = vec!["1", "2", "3"];
        let parse_all = try_map_all(values, |s| s.parse::<i32>());
        assert_eq!(parse_all, Ok(vec![1, 2, 3]));
        
        let values_with_error = vec!["1", "abc", "3"];
        let parse_with_error = try_map_all(values_with_error, |s| s.parse::<i32>());
        assert!(parse_with_error.is_err());
    }
}