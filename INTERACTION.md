# Internal Service Call
ServiceCommunicator is use to communicate with local services, it dynamically detect service from header and then prepare a call
and get resposen

```rust
#[derive(Serialize, Deserialize)]
struct RoleClaimsRequest {
    user_id: String,
}

#[derive(Serialize, Deserialize)]
struct MyResponse {
    message: String,
}

#[get("")]
pub async fn get_all(connection: web::Data<PgPool>, connector: web::Data<ServiceCommunicator>) -> Result<impl Responder, actix_web::Error> {
    let payload = RoleClaimsRequest { user_id: "Pakistan".to_string() };
    let response = connector.call_service("api/auth", Method::POST, Some(payload)).await;
    let result = connector.get_data::<MyResponse>(response).await?;

    let entities = PersonalInformationsService::get_all(connection.as_ref().clone()).await;
    Ok(HttpResponse::Ok().json(entities))
}
```