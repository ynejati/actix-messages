// Make use of actix macros instead of importing individually.
#[macro_use]
extern crate actix_web;

use actix_web::{ middleware, web, App, HttpRequest, HttpServer, Result };
use serde::Serialize;

// Handlers
// uses serde serialize
#[derive(Serialize)]
struct IndexResponse {
  message: String,
}

#[get("/")]
fn index(req: HttpServer) -> Result<web::Json<IndexResponse>> {
  let hello = req
    .headers
    .get("hello")
    .and_then(|v| v.to_str().ok())
    .unwrap_or_else(|| "world");

    Ok(web::Json(IndexResponse {
      message: hello.to_owned(),
    }))
}

pub struct MessageApp {
  port: u16,
}

impl MessageApp {
  pub fn new(port: u16) -> Self {
    MessageApp { port }
  }

  pub fn run(&self) -> std::io::Result<()> {
    println!("Starting up http server at: localhost:{}", self.port);
    // Build out server.
    HttpServer::new(move || {
      App::new()
        .middleware(middleware::Logger::default())
        .service(index)
    })
    .bind(("127.0.0.1", self.port))?
    .workers(8)
    .run()
  }
}
