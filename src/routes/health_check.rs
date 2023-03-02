    use actix_web::HttpResponse;
    use actix_web::Responder;
    use actix_web::get;

    #[get("/health_check")]
    pub(crate) async fn health_check() -> impl Responder {
        HttpResponse::Ok()
    }