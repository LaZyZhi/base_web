use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "rs_employee01", schema_name = "material")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub empid: i64,

    #[sea_orm(unique)]
    pub empno: Option<String>,

    pub empname: Option<String>,

    pub deptid: Option<i64>,

    pub deptno: Option<String>,

    pub deptname: Option<String>,

    pub dutyname: Option<String>,

    pub postname: Option<String>,

    pub idtype: Option<i16>,

    pub idcardno: Option<String>,

    pub idcardsn: Option<String>,

    pub jobtitle: Option<String>,

    pub position: Option<String>,

    pub ethnicgroup: Option<String>,

    pub mealclass: Option<String>,

    #[sea_orm(default_value = "0")]
    pub gender: i16,

    pub idcardaddr: Option<String>,

    pub mobileno: Option<String>,

    pub entrydate: Option<NaiveDate>,

    pub resigndate: Option<NaiveDate>,

    pub birthday: Option<NaiveDate>,

    pub factbirth: Option<NaiveDate>,

    pub remark: Option<String>,

    pub statusid: Option<i16>,

    #[sea_orm(default_value = "1")]
    pub passtype: i16,

    pub accgrpid: Option<i64>,

    pub interviewee: Option<i64>,

    pub nexempno: Option<String>,

    pub accgroup: Option<i64>,

    pub luser: Option<String>,

    pub ltime: Option<chrono::NaiveDateTime>,

    pub psid: Option<String>,

    pub lsempno: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// 检查员工是否在职
    pub fn is_active(&self) -> bool {
        self.statusid.map(|s| s == 1).unwrap_or(false) &&
            self.resigndate.is_none()
    }
}