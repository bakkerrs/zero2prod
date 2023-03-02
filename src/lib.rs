pub mod configuration;
pub mod routes;
pub mod startup;

use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
// use env_logger::Env;
use std::io;
use std::net::TcpListener;

use routes::*;

pub fn run(listener: TcpListener) -> Result<Server, io::Error>  {
    // env_logger::init_from_env(Env::default().default_filter_or("info"));
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(health_check)
            .service(subscribe)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
