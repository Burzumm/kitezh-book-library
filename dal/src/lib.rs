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
    use async_recursion::async_recursion;
    use async_trait::async_trait;

    use log;
    use mongodb::bson::{doc};
    use mongodb::options::{ClientOptions, Credential, FindOptions};
    use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
    use mongodb::{Client, Collection, Database};

    use std::time::Duration;
    use uuid::Uuid as uuid;

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

        async fn find_by_id<'a>(&self, id: uuid)
            -> Result<Option<BookItem>, mongodb::error::Error>;

        async fn find_by_name<'a>(
            &self,
            name: String,
        ) -> Result<Box<Vec<BookItem>>, mongodb::error::Error>;

        async fn find_all(
            &self,
            take: u64,
            skip: u64,
        ) -> Result<Box<Vec<BookItem>>, mongodb::error::Error>;

        async fn update(&self, book_item: &BookItem)
            -> Result<UpdateResult, mongodb::error::Error>;

        async fn delete(&self, _id: uuid) -> Result<DeleteResult, mongodb::error::Error>;

        async fn add(&self, book_item: &BookItem)
            -> Result<InsertOneResult, mongodb::error::Error>;

        // async fn create_client(&self) -> Database;

        // async fn get_book_items_collection(&self) -> Collection<BookItem>;
    }

    pub struct BookItemRepository {
        database_uri: String,
        database_name: String,
        database_user: String,
        database_password: String,
    }

    impl BookItemRepository {
        #[async_recursion]
        async fn get_book_items_collection(&self) -> Collection<BookItem> {
            let _db = self.create_client().await;
            return _db.collection(&CollectionNames::BookItem.to_string());
        }

        #[async_recursion]
        async fn create_client(&self) -> Database {
            let mut options = ClientOptions::parse(&self.database_uri)
                .await
                .unwrap_or_else(|error| {
                    panic!("{}", format!("error parse database uri: {}", error))
                });
            options.connect_timeout = Some(Duration::from_secs(5));
            options.default_database = Some(self.database_name.clone());
            let credential = Credential::builder();
            let credential = credential
                .source(Some(self.database_name.clone()))
                .username(Some(self.database_user.clone()))
                .password(Some(self.database_password.clone()));
            options.credential = Some(credential.build());
            let client = &Client::with_options(options).unwrap_or_else(|error| {
                panic!("{}", format!("error connect to database: {}", error))
            });
            log::info!("create database client");
            return client.database(&self.database_name);
        }
    }

    #[async_trait]
    impl BookItemRepositoryTrait for BookItemRepository {
        async fn add(
            &self,
            book_item: &BookItem,
        ) -> Result<InsertOneResult, mongodb::error::Error> {
            let collection = self.get_book_items_collection().await;
            return collection.insert_one(book_item, None).await;
        }

        async fn delete(&self, _id: uuid) -> Result<DeleteResult, mongodb::error::Error> {
            let collection = self.get_book_items_collection().await;
            return collection
                .delete_one(doc! {"_id": _id.to_string()}, None)
                .await;
        }

        async fn find_by_id<'a>(
            &self,
            id: uuid,
        ) -> Result<Option<BookItem>, mongodb::error::Error> {
            let collection = self.get_book_items_collection().await;
            return collection
                .find_one(
                    doc! {
                    "_id": &id.to_string() },
                    None,
                )
                .await;
        }

        async fn find_by_name<'a>(
            &self,
            name: String,
        ) -> Result<Box<Vec<BookItem>>, mongodb::error::Error> {
            let collection = self.get_book_items_collection().await;
            let mut result = Vec::new();
            let mut cursor = collection.find(doc! {"name" : name}, None).await?;
            while cursor.advance().await? {
                result.push(cursor.deserialize_current()?);
            }
            return Ok(Box::new(result));
        }

        async fn find_all(
            &self,
            _take: u64,
            _skip: u64,
        ) -> Result<Box<Vec<BookItem>>, mongodb::error::Error> {
            let collection = self.get_book_items_collection().await;
            let mut result = Vec::new();
            let find_options = FindOptions::builder()
                .limit(Some(_take as i64))
                .skip(Some(_skip))
                .sort(doc! { "name": 1 })
                .build();
            let mut cursor = collection.find(None, find_options).await?;
            while cursor.advance().await? {
                result.push(cursor.deserialize_current()?);
            }
            return Ok(Box::new(result));
        }

        async fn update(
            &self,
            book_item: &BookItem,
        ) -> Result<UpdateResult, mongodb::error::Error> {
            let collection = self.get_book_items_collection().await;
            println!("{}", &book_item.id.to_string());
            return collection
                .update_one(
                    doc! {"_id": &book_item.id.to_string()},
                    doc! {"$set" :{
                        "name" : &book_item.name,
                        "title" : &book_item.title,
                        "description" : &book_item.description,
                        "url": &book_item.url
                    }

                    },
                    None,
                )
                .await;
        }

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
    }
}
