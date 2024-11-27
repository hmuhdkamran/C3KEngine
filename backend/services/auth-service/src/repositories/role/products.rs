use crate::models::role::products::Products;
use c3k_common::{
    handler::error_display::ParseError,
    interfaces::irepository::{IRepository, Model},
    models::{
        auth::{Auth, UserProducts},
        constants::{
            MESSAGE_CAN_NOT_DELETE_DATA, MESSAGE_CAN_NOT_INSERT_DATA, MESSAGE_CAN_NOT_UPDATE_DATA,
        },
    },
};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    Arguments, PgPool, Postgres, Row,
};
use std::error::Error as StdError;

pub struct ProductsRepository {}

impl IRepository<Products> for ProductsRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<Products>, Box<dyn StdError>> {
        let result = sqlx::query(&Products::build_select_string(
            Products::TABLE,
            &Products::COLUMNS_ARRAY,
            None,
        ))
        .map(|row: PgRow| Products::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<Products>, Box<dyn StdError>> {
        let result = sqlx::query(&Products::build_select_string(
            Products::TABLE,
            &Products::COLUMNS_ARRAY,
            Some(filter),
        ))
        .map(|row: PgRow| Products::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &Products) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &Products::build_insert_string(Products::TABLE, &Products::COLUMNS_ARRAY),
            entity.get_args(),
        )
        .execute(&connection)
        .await
        .map(|result| {
            if result.rows_affected() > 0 {
                Ok(true)
            } else {
                Err(Box::new(ParseError::from(MESSAGE_CAN_NOT_INSERT_DATA)) as Box<dyn StdError>)
            }
        })
        .map_err(|e| Box::new(e) as Box<dyn StdError>)
        .unwrap_or_else(|e| Err(e))
    }

    async fn update(connection: PgPool, entity: &Products) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &Products::build_update_string(Products::TABLE, &Products::COLUMNS_ARRAY, Products::PK),
            entity.get_args(),
        )
        .execute(&connection)
        .await
        .map(|result| {
            if result.rows_affected() > 0 {
                Ok(true)
            } else {
                Err(Box::new(ParseError::from(MESSAGE_CAN_NOT_UPDATE_DATA)) as Box<dyn StdError>)
            }
        })
        .map_err(|e| Box::new(e) as Box<dyn StdError>)
        .unwrap_or_else(|e| Err(e))
    }

    async fn delete(connection: PgPool, id: &String) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(id);

        sqlx::query_with(
            &Products::build_delete_string(Products::TABLE, Products::PK),
            args,
        )
        .execute(&connection)
        .await
        .map(|result| {
            if result.rows_affected() > 0 {
                Ok(true)
            } else {
                Err(Box::new(ParseError::from(MESSAGE_CAN_NOT_DELETE_DATA)) as Box<dyn StdError>)
            }
        })
        .map_err(|e| Box::new(e) as Box<dyn StdError>)
        .unwrap_or_else(|e| Err(e))
    }
}

impl ProductsRepository {
    pub async fn get_products(
        connection: PgPool,
        username: &String,
    ) -> Result<Vec<UserProducts>, Box<dyn StdError>> {
        let query = if !username.is_empty() {
            format!(
                r#"SELECT pr."ProductId", pr."Abbreviation", pr."FullName", pr."Description", pr."Icon", pr."FrontendIp", pr."FrontendPort" 
            FROM "Role"."UserApplications" pr 
            WHERE pr."Username" = '{}'"#,
                username
            )
        } else {
            let new_username = r#"WHERE pr."Username" IS NOT NULL"#;
            format!(
                r#"SELECT pr."ProductId", pr."Abbreviation", pr."FullName", pr."Description", pr."Icon", pr."FrontendIp", pr."FrontendPort" 
            FROM "Role"."UserApplications" pr {}"#,
            new_username
            )
        };

        let result = sqlx::query(query.as_str())
            .map(|row: PgRow| UserProducts::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    pub async fn get_claims(
        connection: PgPool,
        username: &String,
        product: &String,
    ) -> Result<Vec<Auth>, Box<dyn StdError>> {
        let query = if !product.is_empty() {
            format!(
                r#"SELECT uar."RouteName", uar."Operation" 
                   FROM "Role"."UserApplicationRoles" uar 
                   WHERE uar."Username"='{}' AND uar."Product"='{}'"#,
                username, product
            )
        } else {
            format!(
                r#"SELECT ur."RouteName", ur."Operation" 
                   FROM "Role"."UserRoles" ur 
                   WHERE ur."Username"='{}'"#,
                username
            )
        };

        // Execute the query
        let result = sqlx::query(query.as_str())
            .map(|row: PgRow| Auth::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }
}
