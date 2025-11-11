use crate::entities::permission::rs_employee01;
use crate::entities::prelude::RsEmployee1;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn query_employee_by_emp_no(
    emp_no: &str,
    db: &DatabaseConnection,
) -> Option<rs_employee01::Model> {
    match RsEmployee1::find()
        .filter(rs_employee01::Column::Empno.eq(emp_no))
        .one(db)
        .await
    {
        Ok(Some(employee)) => {
            tracing::info!("Successfully found employee with emp_no {}: {:?}", emp_no, employee);
            Some(employee)
        },
        Ok(None) => {
            tracing::warn!("Employee with emp_no {} not found", emp_no);
            None
        }
        Err(e) => {
            tracing::error!("with emp_no {}: {}", emp_no, e);
            None
        }
    }
}