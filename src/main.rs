pub(crate) use actix_web::{get, App, HttpResponse, HttpServer, Responder};
mod quotes;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/getquote")]
async fn getquotes(_req_body: String) -> impl Responder {
    let mut quotes: quotes::Quotes = quotes::Quotes::new();
    HttpResponse::Ok().body(String::from(quotes.get_random_quote()))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(getquotes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}