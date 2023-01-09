mod dto;

pub mod book_items_handlers {

    use actix_web::web::Json;
    use actix_web::{get, post, HttpResponse};
    use uuid::Uuid;

    use dal::models::BookItem;
    use dal::repositories::BookItemRepositoryTrait;

    use crate::dto::BookItemDto;

    #[get("/book_items")]
    pub async fn get_book_items() -> HttpResponse {
        let repo = dal::repositories::BookItemRepository::new(
            "mongodb://localhost:27017".to_string(),
            "test".to_string(),
            "admin".to_string(),
            "admin".to_string(),
        );
        let result = repo.find_all(0, 0).await;
        match result {
            Ok(items) => {
                log::info!("get book items success");
                HttpResponse::Ok().json(items)
            }
            Err(err) => {
                log::warn!("book added failed {}", err);
                HttpResponse::InternalServerError().body(err.to_string())
            }
        }
    }

    #[post("/book_items")]
    pub async fn add_book_item(form: Json<BookItemDto>) -> HttpResponse {
        let repo = dal::repositories::BookItemRepository::new(
            "mongodb://localhost:27017".to_string(),
            "test".to_string(),
            "admin".to_string(),
            "admin".to_string(),
        );
        let result = repo
            .add(&BookItem {
                _id: Uuid::new_v4(),
                url: form.url.clone(),
                title: form.title.clone(),
                description: form.description.clone(),
            })
            .await;
        match result {
            Ok(_) => {
                log::info!("book added success");
                HttpResponse::Ok().body("book added")
            }
            Err(err) => {
                log::warn!("book added failed {}", err);
                HttpResponse::InternalServerError().body(err.to_string())
            }
        }
    }
}

pub mod base_handlers {
    use actix_files::NamedFile;
    use actix_session::Session;
    use actix_web::{
        get,
        http::{header::ContentType, Method, StatusCode},
        Either, HttpRequest, HttpResponse, Responder, Result,
    };

    /// simple index handler
    #[get("/home")]
    pub async fn home(req: HttpRequest, session: Session) -> Result<HttpResponse> {
        println!("{req:?}");

        // session
        let mut counter = 1;
        if let Some(count) = session.get::<i32>("counter")? {
            println!("SESSION value: {count}");
            counter = count + 1;
        }

        // set counter to session
        session.insert("counter", counter)?;

        // response
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::plaintext())
            .body(include_str!("../../static/index.html")))
    }

    pub async fn default_handler(req_method: Method) -> Result<impl Responder> {
        match req_method {
            Method::GET => {
                let file = NamedFile::open("static/404.html")?
                    .customize()
                    .with_status(StatusCode::NOT_FOUND);
                Ok(Either::Left(file))
            }
            _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
        }
    }

    /// favicon handler
    #[get("/favicon")]
    pub async fn favicon() -> Result<impl Responder> {
        Ok(NamedFile::open("static/favicon.ico")?)
    }
}
