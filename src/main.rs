pub(crate) use actix_web::{get, App, HttpResponse, HttpServer, Responder};
mod quotes;

/// qetquotes is bound with an actix decorator to a get request at the webroot
/// it instantiates a quotes object and then returns the formatted string that
///  is generated by the object method named get_random_quote
#[get("/")]
async fn getquotes(_req_body: String) -> impl Responder {
    let mut quotes: quotes::Quotes = quotes::Quotes::new();
    HttpResponse::Ok().body(String::from(quotes.get_random_quote()))
}
/// the main function is the entrypoint to this application. It creates a new actix web object, binds to the
/// available services and starts the web server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(getquotes))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
