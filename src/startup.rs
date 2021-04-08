//! src/startup.rs

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use crate::routes::{health_checker,subscribe};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_checker))
            .route("/subscriptions", web::post().to(subscribe))
    })
        .listen(listener)?
        .run();
    Ok(server)
}
