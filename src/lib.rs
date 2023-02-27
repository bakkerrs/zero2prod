use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use env_logger::Env;
use std::io;
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, io::Error>  {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();

    Ok(server)
}


async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
