#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

// use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod api_error;
mod db_connection;
mod schema;
mod textinfo;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    // Initialize db connection pool
    db_connection::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().configure(textinfo::init_routes));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting server");
    server.run().await
}
