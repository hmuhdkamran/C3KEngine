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
#[serde(rename_all = "PascalCase")]
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
    pub const PK: &'static str = "SalarieId";
    pub const COLUMNS_ARRAY: [&'static str; 13] = ["SalarieId","SalaryTypeId","EmployeeId","EmployeeJobInfoId","AllowanceAmount","DeductionAmount","NetAmount","YearId","WeekId","FortNightId","Freezed","StatusId","MonthId"];

    pub fn get_id(&self) -> Uuid {
        self.salarie_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.salarie_id.clone());
let _ = args.add(self.salary_type_id.clone());
let _ = args.add(self.employee_id.clone());
let _ = args.add(self.employee_job_info_id.clone());
let _ = args.add(self.allowance_amount.clone());
let _ = args.add(self.deduction_amount.clone());
let _ = args.add(self.net_amount.clone());
let _ = args.add(self.year_id.clone());
let _ = args.add(self.week_id.clone());
let _ = args.add(self.fort_night_id.clone());
let _ = args.add(self.freezed.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.month_id.clone());

        args
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
