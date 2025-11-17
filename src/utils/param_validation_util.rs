use crate::AppError;
use validator::Validate;

/// 校验入参工具，返回包装过的应用错误。
pub async fn validate_param<T>(param: &T) -> Result<(), AppError>
where
    T: Validate,
{
    param
        .validate()
        .map_err(|e| AppError::public(format!("验证失败: {}", e)))?;
    Ok(())
}
