extern crate core;

pub mod models;
pub mod database_collection_name {
    use std::fmt;

    pub enum CollectionNames {
        BookItem,
    }
    impl fmt::Display for CollectionNames {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                CollectionNames::BookItem => write!(f, "BookItem"),
            }
        }
    }
}

pub mod repositories {
    use std::time::Duration;

    
    use async_trait::async_trait;
    use mongodb::bson::Uuid;
    use mongodb::options::{ClientOptions, Credential};
    use mongodb::results::InsertOneResult;
    use mongodb::{Client, Database};

    use crate::database_collection_name::CollectionNames;
    use crate::models::BookItem;

    #[async_trait]
    pub trait BookItemRepositoryTrait {
        fn new(
            database_uri: String,
            database_name: String,
            database_user: String,
            database_password: String,
        ) -> Box<Self>;
        fn get<'a>(&self, id: Uuid) -> &'a BookItem;

        fn get_all(&self, take: i32, skip: i32) -> Vec<BookItem>;

        fn update(&self);

        fn delete(&self);

        async fn create_client(&self) -> Database;

        async fn add(&self, book_item: &BookItem)
            -> Result<InsertOneResult, mongodb::error::Error>;
    }

    pub struct BookItemRepository {
        database_uri: String,
        database_name: String,
        database_user: String,
        database_password: String,
    }

    #[async_trait]
    impl BookItemRepositoryTrait for BookItemRepository {
        fn new(
            database_uri: String,
            database_name: String,
            database_user: String,
            database_password: String,
        ) -> Box<BookItemRepository> {
            Box::new(BookItemRepository {
                database_uri,
                database_name,
                database_user,
                database_password,
            })
        }

        fn get<'a>(&self, _id: Uuid) -> &'a BookItem {
            todo!()
        }
        fn get_all(&self, _take: i32, _skip: i32) -> Vec<BookItem> {
            todo!()
        }
        fn update(&self) {
            todo!()
        }
        fn delete(&self) {
            todo!()
        }
        async fn create_client(&self) -> Database {
            let mut options = ClientOptions::parse(&self.database_uri)
                .await
                .unwrap_or_else(|error| {
                    panic!("{}", format!("error parse database uri: {}", error))
                });
            options.connect_timeout = Some(Duration::from_secs(5));
            options.default_database = Some(self.database_name.clone());
            let credential = Credential::builder();
            let credential = credential.source(Some(self.database_name.clone()));
            let credential = credential.username(Some(self.database_user.clone()));
            let credential = credential.password(Some(self.database_password.clone()));
            options.credential = Some(credential.build());
            let client = Client::with_options(options).unwrap_or_else(|error| {
                panic!("{}", format!("error connect to database: {}", error))
            });
            return client.database(&self.database_name);
        }

        async fn add(
            &self,
            book_item: &BookItem,
        ) -> Result<InsertOneResult, mongodb::error::Error> {
            let db = &self.create_client().await;
            let collection = db.collection::<BookItem>(&CollectionNames::BookItem.to_string());
            return collection.insert_one(book_item, None).await;
        }
    }
}
