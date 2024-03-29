pub mod auth;
pub mod posts;

use actix_files as fs;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use auth::{auth_status, login, logout};
use posts::{
    domain::{
        author_repository::AuthorRepository,
        model::{author::Author, posts::Post},
        post_repository::PostRepository,
    },
    infrastructure::{
        http::routes::*,
        repository::{
            author_repository::InMemoryAuthorRepository, post_repository::InMemoryPostRepository,
        },
    },
};
use std::{collections::HashMap, sync::Mutex};

use actix_files::NamedFile;
use actix_web::{cookie::Key, get, web::Data, App, HttpServer};

use chrono::Utc;

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("index.html")?)
}

// The secret key would usually be read from a configuration file/environment variables.
fn get_secret_key() -> Key {
    let key: &Vec<u8> = &(0..64).collect();
    Key::from(key)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                get_secret_key(),
            ))
            .app_data::<Data<Box<dyn PostRepository>>>(Data::new(Box::new(
                InMemoryPostRepository {
                    posts: Mutex::new(HashMap::from([(
                        "1".to_string(),
                        Post {
                            id: "1".to_string(),
                            published_at: Utc::now(),
                            author: "1".to_string(),
                            title: "Hello, world!".to_string(),
                            content: "Hello, world!".to_string(),
                        },
                    )])),
                },
            )))
            .app_data::<Data<Box<dyn AuthorRepository>>>(Data::new(Box::new(
                InMemoryAuthorRepository {
                    authors: Mutex::new(HashMap::from([(
                        "1".to_string(),
                        Author {
                            id: "1".to_string(),
                            name: "John Doe".to_string(),
                        },
                    )])),
                },
            )))
            .service(index)
            .service(fs::Files::new("/static", "./resources/public").show_files_listing())
            .service(add_post_button_endpoint)
            .service(list_posts_endpoint)
            .service(edit_post_endpoint)
            .service(get_post_endpoint)
            .service(create_post_endpoint)
            .service(update_post_endpoint)
            .service(auth_status)
            .service(login)
            .service(logout)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
