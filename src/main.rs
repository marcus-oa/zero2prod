//! main.ls

use zero2prod::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    // Bubble up the io::Error if we failed to bind
    // the address
    // Otherwise call .await on our server
    run(listener)?.await
}
