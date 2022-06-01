use actix_web::{error, get, web, App, FromRequest, HttpResponse, Responder, Result};

use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    url: String,
    file: String,
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello!")
}

#[get("/import_profile")]
pub async fn import_profile(
    query: web::Query<Info>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let mut builder = reqwest::ClientBuilder::new();
    println!("{}", query.url);
    let resp = builder.build()?.get(query.url.clone()).send().await?;
    let data = resp.text_with_charset("utf-8").await?;

    println!("{}", data);
    Ok(HttpResponse::Ok().body(format!("Welcome {}!", query.file)))
}
