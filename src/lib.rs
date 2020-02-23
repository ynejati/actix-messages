// Make use of actix macros instead of importing individually.
#[macro_use]
extern crate actix_web;

use actix_web::{ middleware, web, App, HttpServer, Result };
use serde::Serialize;
use std::cell::Cell;
// Tools needed to work with a usize that can be modified atomically and thefore threadsafe.
use std::sync::atomic::{AtomicUsize, Ordering};
// Tools to safely share and mutate things that are not atomic accross multiple threads.
use std::sync::{Arc, Mutex};

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

// Handlers
// uses serde serialize
#[derive(Serialize)]
struct IndexResponse {
  server_id: usize,
  request_count: usize,
  messages: Vec<String>
}

// Each worker thread will get own instance of this struct.
struct AppState {
  server_id: usize,
  request_count: Cell<usize>,
  messages: Arc<Mutex<Vec<String>>>,
}

#[get("/")]
fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
  let request_count = state.request_count.get() + 1;
  state.request_count.set(request_count);
  let ms = state.messages.lock().unwrap();

  Ok(web::Json(IndexResponse {
    server_id: state.server_id,
    request_count,
    messages: ms.clone()
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
    // Shared messages vector among workers.
    let messages = Arc::new(Mutex::new(vec![]));
    println!("Starting http server: 127.0.0.1:{}", self.port);
    // Build out server.
    HttpServer::new(move || {
      App::new()
        .data(AppState {
          server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
          request_count: Cell::new(0),
          messages: messages.clone(),
        })
        .wrap(middleware::Logger::default())
        .service(index)
    })
    .bind(("127.0.0.1", self.port))?
    .workers(8)
    .run()
  }
}
