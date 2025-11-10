use crate::AppError;
use validator::Validate;

pub async fn validate_param<T>(param: T) -> Result<(), AppError> 
where 
    T: Validate 
{
    param.validate().map_err(|e| AppError::public(format!("验证失败: {}", e)))?;
    Ok(())
}