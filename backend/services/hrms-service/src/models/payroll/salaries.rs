use c3k_common::interfaces::irepository::Model;
use serde::{Deserialize, Serialize};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    Arguments, Error, PgPool, Postgres, Row,
    types::chrono::{DateTime, Utc},
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Salaries {
    pub salarie_id: Uuid,
pub salary_type_id: Uuid,
pub employee_id: Uuid,
pub employee_job_info_id: Uuid,
pub allowance_amount: f64,
pub deduction_amount: f64,
pub net_amount: f64,
pub year_id: Uuid,
pub week_id: Uuid,
pub fort_night_id: Uuid,
pub freezed: bool,
pub status_id: Uuid,
pub month_id: Uuid

}

impl Salaries {
    pub const TABLE: &'static str = r#""Payroll"."Salaries""#;
    pub const PK: &'static str = r#"SalarieId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""SalarieId","SalaryTypeId","EmployeeId","EmployeeJobInfoId","AllowanceAmount","DeductionAmount","NetAmount","YearId","WeekId","FortNightId","Freezed","StatusId","MonthId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""SalarieId"=$1,"SalaryTypeId"=$2,"EmployeeId"=$3,"EmployeeJobInfoId"=$4,"AllowanceAmount"=$5,"DeductionAmount"=$6,"NetAmount"=$7,"YearId"=$8,"WeekId"=$9,"FortNightId"=$10,"Freezed"=$11,"StatusId"=$12,"MonthId"=$13 WHERE "SalarieId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.salarie_id.clone()
    }

    pub fn new(salarie_id: Uuid,salary_type_id: Uuid,employee_id: Uuid,employee_job_info_id: Uuid,allowance_amount: f64,deduction_amount: f64,net_amount: f64,year_id: Uuid,week_id: Uuid,fort_night_id: Uuid,freezed: bool,status_id: Uuid,month_id: Uuid) -> Self {
        Self {
            salarie_id,
salary_type_id,
employee_id,
employee_job_info_id,
allowance_amount,
deduction_amount,
net_amount,
year_id,
week_id,
fort_night_id,
freezed,
status_id,
month_id

        }
    }
}

impl PartialEq for Salaries {
    fn eq(&self, other: &Self) -> bool {
        self.salarie_id == other.salarie_id
    }
}

impl Model for Salaries {
    fn from_row(row: &PgRow) -> Salaries
    where
        Self: Sized,
    {
        let salarie_id = row.get("SalarieId");
let salary_type_id = row.get("SalaryTypeId");
let employee_id = row.get("EmployeeId");
let employee_job_info_id = row.get("EmployeeJobInfoId");
let allowance_amount = row.get("AllowanceAmount");
let deduction_amount = row.get("DeductionAmount");
let net_amount = row.get("NetAmount");
let year_id = row.get("YearId");
let week_id = row.get("WeekId");
let fort_night_id = row.get("FortNightId");
let freezed = row.get("Freezed");
let status_id = row.get("StatusId");
let month_id = row.get("MonthId");


        Self {
            salarie_id,
salary_type_id,
employee_id,
employee_job_info_id,
allowance_amount,
deduction_amount,
net_amount,
year_id,
week_id,
fort_night_id,
freezed,
status_id,
month_id

        }
    }
}
