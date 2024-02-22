use crate::routes::{health_check, subscribe};
use std::net::TcpListener;
use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;

pub fn run(tcp_listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
        .listen(tcp_listener)?
        .run();

    Ok(server)
}