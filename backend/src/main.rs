use anyhow::Result;
use actix_web::{web, App, HttpServer};

pub mod routes;

use routes::user::{create_user, sign_in};

#[actix_web::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let store = db::Store::new().await?;
    HttpServer::new(move || {
        actix_web::App::new()
        .service(web::scope("/api/v1")
            .service(web::resource("/signup").route(web::post().to(create_user)))
            .service(web::resource("/signin").route(web::post().to(sign_in))))
            .app_data(web::Data::new(store.clone()))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}