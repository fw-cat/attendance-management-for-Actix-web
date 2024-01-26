use std::collections::HashMap;

use actix_web::{
  body::BoxBody,
  dev::ServiceResponse,
  web, App, HttpServer, Responder, Result,
  http::{
    header::ContentType, StatusCode
  },
  middleware::{
    ErrorHandlerResponse, ErrorHandlers
  },
};
use actix_web_lab::respond::Html;
use askama::Template;


#[derive(Template)]
#[template(path = "user.html")]
struct HelloTemplate<'a> {
  name: &'a str,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

async fn index(query: web::Query<HashMap<String, String>>) -> Result<impl Responder> {
  let html = IndexTemplate.render().expect("template should be valid");
  Ok(Html(html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(move || {
    App::new()
      .route("/", web::get().to(index))
  })
  .bind(("0.0.0.0", 8080))?
  .run().await
}