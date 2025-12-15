use actix_web::{web, Result};
use db::Store;
use db::models::user::{CreateUserRequest, CreateUserResponse, GetUserRequest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignInResponse {
    pub token: String,
}

pub async fn create_user(data: web::Data<Store>, request: web::Json<CreateUserRequest>) -> Result<web::Json<CreateUserResponse>> {
    let store = data.into_inner();
    let user = store.create_user(request.into_inner()).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    Ok(web::Json(user))
}

pub async fn sign_in(data: web::Data<Store>, request: web::Json<CreateUserRequest>) -> Result<web::Json<SignInResponse>> {
    let store = data.into_inner();
    let user = store.get_user(GetUserRequest { username: request.into_inner().username }).await.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    let token = String::from(""); // add jwt logic here
    Ok(web::Json(SignInResponse { token: token }))
}