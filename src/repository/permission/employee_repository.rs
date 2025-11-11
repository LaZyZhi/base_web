use crate::{entities::permission::rs_employee01, utils::error_util};
use crate::entities::prelude::RsEmployee1;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use crate::common::api_response::AppResult;

pub async fn query_employee_by_emp_no(
    emp_no: &str,
    db: &DatabaseConnection,
) -> AppResult<Option<rs_employee01::Model>> {
    match RsEmployee1::find()
        .filter(rs_employee01::Column::Empno.eq(emp_no))
        .one(db)
        .await
    {
        Ok(Some(employee)) => {
            tracing::info!("Successfully found employee with emp_no {}: {:?}", emp_no, employee);
            Ok(Some(employee))
        },
        Ok(None) => {
            tracing::warn!("Employee with emp_no {} not found", emp_no);
            Ok(None)
        }
        Err(e) => {
            tracing::error!("Database error when querying employee with emp_no {}: {}", emp_no, e);
            Err(error_util::system_error())
        }
    }
}