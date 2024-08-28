use futures::future::try_join_all;
use serde::Deserialize;
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, Row,
};
use std::error::Error as StdError;

#[derive(Debug, Deserialize, Clone)]
pub struct Column {
    pub ordinal_position: i32,
    pub column_name: String,
    pub data_type: String,
    pub type_detail: Option<String>,
    pub is_nullable: String,
    pub is_primary_key: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Table {
    pub table_name: String,
    pub columns: Vec<Column>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Schema {
    pub schema_name: String,
    pub tables: Vec<Table>,
}

pub struct PostgreSqlHandler;

impl PostgreSqlHandler {
    pub async fn get_schemas(connection_string: &str) -> Result<Vec<Schema>, Box<dyn StdError>> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_string)
            .await?;

        let schema_query = "SELECT schema_name FROM information_schema.schemata WHERE schema_name NOT ILIKE 'pg_%' AND schema_name NOT ILIKE 'information_%';";
        let schemas: Vec<Schema> = sqlx::query(schema_query)
            .map(|row: PgRow| Schema {
                schema_name: row.get("schema_name"),
                tables: Vec::new(),
            })
            .fetch_all(&pool)
            .await?
            .into_iter()
            .collect();

        let mut futures = Vec::new();

        for schema in schemas {
            let pool = pool.clone();
            let schema_name = schema.schema_name.clone();
            let future = async move {
                let tables = PostgreSqlHandler::get_tables(&pool, &schema_name).await?;
                Ok(Schema {
                    schema_name,
                    tables,
                })
            };
            futures.push(future);
        }

        try_join_all(futures).await
    }

    async fn get_tables(pool: &PgPool, schema_name: &str) -> Result<Vec<Table>, Box<dyn StdError>> {
        let table_query = "SELECT table_name FROM information_schema.tables WHERE table_schema = $1 AND table_type = 'BASE TABLE';";
        let tables: Vec<Table> = sqlx::query(table_query)
            .bind(schema_name)
            .map(|row: PgRow| Table {
                table_name: row.get("table_name"),
                columns: Vec::new(),
            })
            .fetch_all(pool)
            .await?
            .into_iter()
            .collect();

        let mut futures = Vec::new();

        for mut table in tables {
            let pool_ref = pool.clone();
            let schema_name = schema_name.to_string();
            let table_name = table.table_name.clone();
            let future = async move {
                table.columns =
                    PostgreSqlHandler::get_columns(&pool_ref, &schema_name, &table_name).await?;
                Ok(table)
            };
            futures.push(future);
        }

        try_join_all(futures).await
    }

    async fn get_columns(
        pool: &PgPool,
        schema_name: &str,
        table_name: &str,
    ) -> Result<Vec<Column>, Box<dyn StdError>> {
        let column_query = r#"
            SELECT 
                c.ordinal_position,
                c.column_name, 
                c.data_type, 
                COALESCE(c.character_maximum_length::text, 
                         c.numeric_precision || '(' || c.numeric_scale || ')') AS type_detail,
                c.is_nullable,
                CASE 
                    WHEN tc.constraint_type = 'PRIMARY KEY' THEN 'YES'
                    ELSE 'NO'
                END AS is_primary_key
            FROM 
                information_schema.columns c
            LEFT JOIN 
                information_schema.key_column_usage kcu
                ON c.table_name = kcu.table_name
                AND c.table_schema = kcu.table_schema
                AND c.column_name = kcu.column_name
            LEFT JOIN 
                information_schema.table_constraints tc
                ON kcu.table_name = tc.table_name
                AND kcu.table_schema = tc.table_schema
                AND kcu.constraint_name = tc.constraint_name
                AND tc.constraint_type = 'PRIMARY KEY'
            WHERE 
                c.table_schema = $1 
                AND c.table_name = $2
            ORDER BY 
                c.ordinal_position;
        "#;

        let columns: Vec<Column> = sqlx::query(column_query)
            .bind(schema_name)
            .bind(table_name)
            .map(|row: PgRow| Column {
                ordinal_position: row.get("ordinal_position"),
                column_name: row.get("column_name"),
                data_type: row.get("data_type"),
                type_detail: row.try_get("type_detail").ok(),
                is_nullable: row.get("is_nullable"),
                is_primary_key: row.get("is_primary_key"),
            })
            .fetch_all(pool)
            .await?;

        Ok(columns)
    }
}
