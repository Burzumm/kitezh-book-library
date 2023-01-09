mod dto;

pub mod book_items_handlers {

    use actix_web::web::{self, Json};
    use actix_web::{delete, get, post, put, HttpResponse};
    use uuid::Uuid;

    use dal::models::BookItem;
    use dal::repositories::BookItemRepositoryTrait;

    use crate::dto::{BookItemDto, UpdateBookItemDto};

    #[get("/book_items")]
    pub async fn get_all_books_items() -> HttpResponse {
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

    #[put("/book_items")]
    pub async fn update_book_item(dto: Json<UpdateBookItemDto>) -> HttpResponse {
        let repo = dal::repositories::BookItemRepository::new(
            "mongodb://localhost:27017".to_string(),
            "test".to_string(),
            "admin".to_string(),
            "admin".to_string(),
        );
        let result = repo
            .update(&BookItem {
                id: dto.id.clone(),
                description: dto.description.clone(),
                name: dto.name.clone(),
                title: dto.title.clone(),
                url: dto.url.clone(),
            })
            .await;
        match result {
            Ok(item) => {
                log::info!("udpate book item success");
                HttpResponse::Ok().json(item)
            }
            Err(err) => {
                log::warn!("udpate book item failed {}", err);
                HttpResponse::InternalServerError().body(err.to_string())
            }
        }
    }

    #[get("/book_items/{id}")]
    pub async fn get_book_item_by_id(id: web::Path<Uuid>) -> HttpResponse {
        let repo = dal::repositories::BookItemRepository::new(
            "mongodb://localhost:27017".to_string(),
            "test".to_string(),
            "admin".to_string(),
            "admin".to_string(),
        );
        let result = repo.find_by_id(id.clone()).await;
        match result {
            Ok(item) => {
                log::info!("get book item by id success");
                HttpResponse::Ok().json(item)
            }
            Err(err) => {
                log::warn!("get book by id failed:{}", err);
                HttpResponse::InternalServerError().body(err.to_string())
            }
        }
    }

    #[delete("/book_items/{id}")]
    pub async fn delete_book_item(id: web::Path<Uuid>) -> HttpResponse {
        let repo = dal::repositories::BookItemRepository::new(
            "mongodb://localhost:27017".to_string(),
            "test".to_string(),
            "admin".to_string(),
            "admin".to_string(),
        );
        let result = repo.delete(id.clone()).await;
        match result {
            Ok(item) => {
                log::info!("delete book item  success");
                HttpResponse::Ok().json(item)
            }
            Err(err) => {
                log::warn!("delete book item by failed:{}", err);
                HttpResponse::InternalServerError().body(err.to_string())
            }
        }
    }

    #[get("/book_items/{name}")]
    pub async fn get_book_item_by_name(name: web::Path<String>) -> HttpResponse {
        let repo = dal::repositories::BookItemRepository::new(
            "mongodb://localhost:27017".to_string(),
            "test".to_string(),
            "admin".to_string(),
            "admin".to_string(),
        );
        let result = repo.find_by_name(name.clone()).await;
        match result {
            Ok(item) => {
                log::info!("get book item by name success");
                HttpResponse::Ok().json(item)
            }
            Err(err) => {
                log::warn!("get book by name failed:{}", err);
                HttpResponse::InternalServerError().body(err.to_string())
            }
        }
    }

    #[post("/book_items")]
    pub async fn add_book_item(dto: Json<BookItemDto>) -> HttpResponse {
        let repo = dal::repositories::BookItemRepository::new(
            "mongodb://localhost:27017".to_string(),
            "test".to_string(),
            "admin".to_string(),
            "admin".to_string(),
        );
        let result = repo
            .add(&BookItem {
                id: Uuid::new_v4(),
                name: dto.name.clone(),
                url: dto.url.clone(),
                title: dto.title.clone(),
                description: dto.description.clone(),
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
