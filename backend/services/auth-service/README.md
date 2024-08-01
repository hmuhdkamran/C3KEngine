# My Rust Web Framework

## Table of Contents
1. [Overview](#overview)
2. [Project Structure](#project-structure)
3. [Repository Layer](#repository-layer)
4. [Service Layer](#service-layer)
5. [Controller Layer](#controller-layer)
6. [API Testing with VSCode REST Client](#api-testing-with-vscode-rest-client)
7. [Getting Started](#getting-started)
8. [Dependencies](#dependencies)
9. [License](#license)

## Overview

This project is a Rust-based web framework designed to provide a clean and modular structure for developing RESTful APIs. It uses Actix Web for the web server, SQLx for database interaction, and implements a typical three-layer architecture: Repository, Service, and Controller.

## Project Structure
my_rust_framework/
│
├── src/
│ ├── controllers/
│ │ ├── role_controller.rs
│ │ └── mod.rs
│ ├── models/
│ │ ├── role.rs
│ │ └── mod.rs
│ ├── repositories/
│ │ ├── role_repository.rs
│ │ └── mod.rs
│ ├── services/
│ │ ├── role_service.rs
│ │ └── mod.rs
│ ├── main.rs
│ └── lib.rs
├── tests/
│ ├── global.api
│ └── user.api
├── Cargo.toml
└── README.md

## Repository Layer

The repository layer is responsible for interacting with the database. It contains functions to execute SQL queries and return the results. Each model has a corresponding repository.

### Example: `role_repository.rs`

```rust
use sqlx::PgPool;
use crate::models::role::Role;

pub struct RoleRepository;

impl RoleRepository {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Role>, sqlx::Error> {
        sqlx::query_as!(Role, "SELECT * FROM roles")
            .fetch_all(pool)
            .await
    }

    // Add other CRUD operations (get_by_filter, add, update, delete)
}
```

## Service Layer

The service layer contains the business logic of the application. It uses the repository layer to perform CRUD operations and any additional logic needed.

### Example: role_service.rs
```rust
use sqlx::PgPool;
use crate::models::role::Role;
use crate::repositories::role_repository::RoleRepository;

pub struct RoleService;

impl RoleService {
    pub async fn get_all(pool: PgPool) -> Result<Vec<Role>, sqlx::Error> {
        RoleRepository::get_all(&pool).await
    }

    // Add other CRUD operations (get_by_filter, add, update, delete)
}
```

## Controller Layer

The controller layer defines the API endpoints and handles HTTP requests. It uses the service layer to perform the necessary operations and returns the appropriate HTTP responses.
### Example: role_controller.rs

```rust
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::services::role_service::RoleService;
use crate::models::role::Role;

#[get("")]
pub async fn get_all(pool: web::Data<PgPool>) -> impl Responder {
    match RoleService::get_all(pool.get_ref().clone()).await {
        Ok(roles) => HttpResponse::Ok().json(roles),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Add other CRUD operations (get_by_filter, add, update, delete)

pub fn role_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/role")
            .service(get_all)
            // Register other CRUD operations
    );
}
```

## API Testing with VSCode REST Client

You can use the VSCode REST Client plugin to test the API endpoints. The requests are organized into user.api files for modularity.

```rust
@host = http://localhost:5000/api

# @name login
POST {{host}}/auth HTTP/1.1
Content-Type: application/json

{
    "username": "admin@sefam.com",
    "password": "P@ssw0rd"
}

### Get All Users
GET {{host}}/role/users
Accept: application/json
Authorization: Bearer {{login.response.body.data}}

### Get User by Id
GET {{host}}/role/users/"UserId"='5a0dbd23-6635-4c0e-a109-c64e437a97f9'
Accept: application/json
Authorization: Bearer {{login.response.body.data}}

### Insert User
POST {{host}}/role/users
Content-Type: application/json
Accept: application/json
Authorization: Bearer {{login.response.body.data}}

{
    "user_id": "5a0dbd23-6635-4c0e-a109-c64e437a97f6",
    "username": "admin@hmk.com",
    "display_name": "Muhammad Kamran",
    "language": "en-US",
    "password": "f412d10cb133926e32b04ab07ca1c817f2e04ebf41d9399fe7d47dff52637f32",
    "salt": "98pBTijN66n7ThTD",
    "status_id": "5ef1874c-b462-4462-b92b-46fdd63c5ba4"
}

### Update User
PUT {{host}}/role/users
Content-Type: application/json
Accept: application/json
Authorization: Bearer {{login.response.body.data}}

{
    "user_id": "5a0dbd23-6635-4c0e-a109-c64e437a97f9",
    "username": "admin@sefam.com",
    "display_name": "Muhammad Kamran",
    "language": "en-US",
    "password": "f412d10cb133926e32b04ab07ca1c817f2e04ebf41d9399fe7d47dff52637f32",
    "salt": "98pBTijN66n7ThTD",
    "status_id": "5ef1874c-b462-4462-b92b-46fdd63c5ba4"
}

### Delete User
DELETE {{host}}/role/users/5a0dbd23-6635-4c0e-a109-c64e437a97f5
Accept: application/json
Authorization: Bearer {{login.response.body.data}}

```