use std::io;

use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    error,
    http::{
        header::{self},
        Method, StatusCode,
    },
    middleware, web, App, HttpRequest, HttpResponse, HttpServer,
};
use mongodb::Client;

use handlers::base_handlers;

static SESSION_SIGNING_KEY: &[u8] = &[0; 64];

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    // random key means that restarting server will invalidate existing session cookies
    let key = actix_web::cookie::Key::from(SESSION_SIGNING_KEY);

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            // enable automatic response compression - usually register this first
            .wrap(middleware::Compress::default())
            // cookie session middleware
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            // enable logger - always register Actix Web Logger middleware last
            .wrap(middleware::Logger::default())
            // register favicon
            .service(base_handlers::favicon)
            // register simple route, handle all methods
            .service(base_handlers::home)
            .service(handlers::book_items_handlers::add_book_item)
            // with path parameters
            // async response body
            .service(
                web::resource("/test").to(|req: HttpRequest| match *req.method() {
                    Method::GET => HttpResponse::Ok(),
                    Method::POST => HttpResponse::MethodNotAllowed(),
                    _ => HttpResponse::NotFound(),
                }),
            )
            .service(web::resource("/error").to(|| async {
                error::InternalError::new(
                    io::Error::new(io::ErrorKind::Other, "test"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            }))
            .service(
                web::resource("/").route(web::get().to(|req: HttpRequest| async move {
                    println!("{req:?}");
                    HttpResponse::Found()
                        .insert_header((header::LOCATION, "index.html"))
                        .finish()
                })),
            )
            // default
            .default_service(web::to(base_handlers::default_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
