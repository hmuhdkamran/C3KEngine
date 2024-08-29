use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use c3k_common::{handler::service_client::ServiceCommunicator, models::config::app_config::get_json};
use c3k_hrms_service::controllers::{
    attendance::attendamce_exclude_employees::attendamce_exclude_employees_routes,
    attendance::attendance_policies::attendance_policies_routes,
    attendance::attendance_statuses::attendance_statuses_routes,
    attendance::employee_shifts::employee_shifts_routes,
    attendance::gazetted_holidays::gazetted_holidays_routes,
    attendance::leave_application_approvals::leave_application_approvals_routes,
    attendance::leave_applications::leave_applications_routes,
    attendance::leave_types::leave_types_routes,
    attendance::overtime_request_approvals::overtime_request_approvals_routes,
    attendance::overtime_requests::overtime_requests_routes, attendance::shifts::shifts_routes,
    attendance::short_leave_apporvals::short_leave_apporvals_routes,
    attendance::short_leaves::short_leaves_routes,
    employee::employee_addresses::employee_addresses_routes,
    employee::employee_bank_infos::employee_bank_infos_routes,
    employee::employee_contacts::employee_contacts_routes,
    employee::employee_document_apporovals::employee_document_apporovals_routes,
    employee::employee_documents::employee_documents_routes,
    employee::employee_educations::employee_educations_routes,
    employee::employee_experiences::employee_experiences_routes,
    employee::employee_id_types::employee_id_types_routes,
    employee::employee_job_infos::employee_job_infos_routes,
    employee::employee_leave_entitlements::employee_leave_entitlements_routes,
    employee::employee_references::employee_references_routes,
    employee::employee_reporting_tos::employee_reporting_tos_routes,
    employee::employee_skills::employee_skills_routes,
    employee::personal_informations::personal_informations_routes,
    employee::spouse_contacts::spouse_contacts_routes, employee::spouses::spouses_routes,
    payroll::allowances::allowances_routes, payroll::deductions::deductions_routes,
    payroll::employee_allowances::employee_allowances_routes,
    payroll::employee_deductions::employee_deductions_routes,
    payroll::employee_expenses::employee_expenses_routes,
    payroll::expense_approvals::expense_approvals_routes,
    payroll::income_tax_slabs::income_tax_slabs_routes,
    payroll::loan_application_approvals::loan_application_approvals_routes,
    payroll::loan_applications::loan_applications_routes,
    payroll::loan_installment_plan_details::loan_installment_plan_details_routes,
    payroll::loan_installment_plans::loan_installment_plans_routes,
    payroll::loan_markup_rates::loan_markup_rates_routes, payroll::loan_types::loan_types_routes,
    payroll::salaries::salaries_routes, payroll::salary_allowaces::salary_allowaces_routes,
    payroll::salary_deductions::salary_deductions_routes,
    payroll::salary_types::salary_types_routes,
    payroll::tax_adjustment_approvals::tax_adjustment_approvals_routes,
    payroll::tax_adjustment_requests::tax_adjustment_requests_routes,
    payroll::travel_request_approvals::travel_request_approvals_routes,
    payroll::travel_request_expenses::travel_request_expenses_routes,
    payroll::travel_requests::travel_requests_routes,
    recruitment::candidate_addresses::candidate_addresses_routes,
    recruitment::candidate_contacts::candidate_contacts_routes,
    recruitment::candidate_spouses::candidate_spouses_routes,
    recruitment::candidates::candidates_routes,
    recruitment::functional_areas::functional_areas_routes,
    recruitment::job_applications::job_applications_routes,
    recruitment::job_experiences::job_experiences_routes, recruitment::job_posts::job_posts_routes,
    recruitment::job_shifts::job_shifts_routes, recruitment::job_types::job_types_routes,
    recruitment::recruitment_job_statuses::recruitment_job_statuses_routes,
    setting::bank_branches::bank_branches_routes, setting::banks::banks_routes,
    setting::cities::cities_routes, setting::countries::countries_routes,
    setting::financial_years::financial_years_routes, setting::fort_nights::fort_nights_routes,
    setting::months::months_routes, setting::states::states_routes, setting::weeks::weeks_routes,
    setting::years::years_routes, setup::address_types::address_types_routes,
    setup::application_status::application_status_routes, setup::blood_groups::blood_groups_routes,
    setup::businesses::businesses_routes, setup::career_levels::career_levels_routes,
    setup::categories::categories_routes, setup::contact_types::contact_types_routes,
    setup::degrees::degrees_routes, setup::departments::departments_routes,
    setup::designations::designations_routes, setup::document_types::document_types_routes,
    setup::employee_types::employee_types_routes, setup::expense_types::expense_types_routes,
    setup::grades::grades_routes, setup::groups::groups_routes, setup::idd_types::idd_types_routes,
    setup::industries::industries_routes, setup::institutes::institutes_routes,
    setup::interests::interests_routes, setup::job_statuses::job_statuses_routes,
    setup::nationalities::nationalities_routes, setup::professoions::professoions_routes,
    setup::relations::relations_routes, setup::religions::religions_routes,
    setup::resign_reasons::resign_reasons_routes, setup::skills::skills_routes,
    setup::spouse_types::spouse_types_routes, setup::statuses::statuses_routes,
};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    Arguments, PgPool, Postgres, Row,
};

use std::io::{Error, ErrorKind};
use std::sync::Arc;

async fn create_db_pool(connection_string: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string)
        .await
        .map_err(|e| {
            eprintln!("Failed to create database pool: {}", e);
            e
        })
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let config = match get_json() {
        Ok(cfg) => Arc::new(cfg),
        Err(err) => {
            eprintln!("Error loading configuration: {}", err);
            return Err(Error::new(ErrorKind::Other, "Configuration error"));
        }
    };

    let service = config.services.iter().find(|f| f.name == "api/hrms").unwrap();

    let addr = format!("{}:{}", service.host, service.port);
    let db_pool = create_db_pool(&service.connection_string)
        .await
        .expect("Failed to create pool");

    let server = HttpServer::new(move || {
        let mut cors = Cors::default()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        cors = cors.allowed_origin(
            format!("http://{}:{}", config.gateway.host, config.gateway.port).as_str(),
        );

        for origin in &config.services {
            cors = cors.allowed_origin(format!("http://{}:{}", origin.host, origin.port).as_str());
        }

        let communicator = ServiceCommunicator::new(config.clone());

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(communicator))
            .configure(attendamce_exclude_employees_routes)
            .configure(attendance_policies_routes)
            .configure(attendance_statuses_routes)
            .configure(employee_shifts_routes)
            .configure(gazetted_holidays_routes)
            .configure(leave_application_approvals_routes)
            .configure(leave_applications_routes)
            .configure(leave_types_routes)
            .configure(overtime_request_approvals_routes)
            .configure(overtime_requests_routes)
            .configure(shifts_routes)
            .configure(short_leave_apporvals_routes)
            .configure(short_leaves_routes)
            .configure(employee_addresses_routes)
            .configure(employee_bank_infos_routes)
            .configure(employee_contacts_routes)
            .configure(employee_document_apporovals_routes)
            .configure(employee_documents_routes)
            .configure(employee_educations_routes)
            .configure(employee_experiences_routes)
            .configure(employee_id_types_routes)
            .configure(employee_job_infos_routes)
            .configure(employee_leave_entitlements_routes)
            .configure(employee_references_routes)
            .configure(employee_reporting_tos_routes)
            .configure(employee_skills_routes)
            .configure(personal_informations_routes)
            .configure(spouse_contacts_routes)
            .configure(spouses_routes)
            .configure(allowances_routes)
            .configure(deductions_routes)
            .configure(employee_allowances_routes)
            .configure(employee_deductions_routes)
            .configure(employee_expenses_routes)
            .configure(expense_approvals_routes)
            .configure(income_tax_slabs_routes)
            .configure(loan_application_approvals_routes)
            .configure(loan_applications_routes)
            .configure(loan_installment_plan_details_routes)
            .configure(loan_installment_plans_routes)
            .configure(loan_markup_rates_routes)
            .configure(loan_types_routes)
            .configure(salaries_routes)
            .configure(salary_allowaces_routes)
            .configure(salary_deductions_routes)
            .configure(salary_types_routes)
            .configure(tax_adjustment_approvals_routes)
            .configure(tax_adjustment_requests_routes)
            .configure(travel_request_approvals_routes)
            .configure(travel_request_expenses_routes)
            .configure(travel_requests_routes)
            .configure(candidate_addresses_routes)
            .configure(candidate_contacts_routes)
            .configure(candidate_spouses_routes)
            .configure(candidates_routes)
            .configure(functional_areas_routes)
            .configure(job_applications_routes)
            .configure(job_experiences_routes)
            .configure(job_posts_routes)
            .configure(job_shifts_routes)
            .configure(job_types_routes)
            .configure(recruitment_job_statuses_routes)
            .configure(bank_branches_routes)
            .configure(banks_routes)
            .configure(cities_routes)
            .configure(countries_routes)
            .configure(financial_years_routes)
            .configure(fort_nights_routes)
            .configure(months_routes)
            .configure(states_routes)
            .configure(weeks_routes)
            .configure(years_routes)
            .configure(address_types_routes)
            .configure(application_status_routes)
            .configure(blood_groups_routes)
            .configure(businesses_routes)
            .configure(career_levels_routes)
            .configure(categories_routes)
            .configure(contact_types_routes)
            .configure(degrees_routes)
            .configure(departments_routes)
            .configure(designations_routes)
            .configure(document_types_routes)
            .configure(employee_types_routes)
            .configure(expense_types_routes)
            .configure(grades_routes)
            .configure(groups_routes)
            .configure(idd_types_routes)
            .configure(industries_routes)
            .configure(institutes_routes)
            .configure(interests_routes)
            .configure(job_statuses_routes)
            .configure(nationalities_routes)
            .configure(professoions_routes)
            .configure(relations_routes)
            .configure(religions_routes)
            .configure(resign_reasons_routes)
            .configure(skills_routes)
            .configure(spouse_types_routes)
            .configure(statuses_routes)
    });

    println!("App is Running on http://{}", addr);
    server.bind(addr)?.run().await
}
