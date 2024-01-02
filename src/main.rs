mod domain;
mod posts;
mod auth;

use actix_files as fs;
use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use auth::{auth_status, login, logout};
use std::{collections::HashMap, sync::Mutex};

use actix_files::NamedFile;
use actix_web::{
    error::{self, InternalError},
    get,
    http::StatusCode,
    web::{self, Data},
    App, HttpServer, Responder, cookie::Key,
    dev::Service as _
};
use askama::Template;
use chrono::{Duration, Utc};
use domain::model::{
    author::Author,
    posts::{Post, Posts},
};
use posts::{edit_post_endpoint, list_posts_endpoint, post_endpoint, post_update_endpoint, create_post_endpoint, add_post_button_endpoint};
use futures_util::future::FutureExt;

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("index.html")?)
}

pub type Authors = HashMap<String, Author>;

// The secret key would usually be read from a configuration file/environment variables.
fn get_secret_key() -> Key {
    let key: &Vec<u8> = &(0..64).collect();
    Key::from(key)
}

#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new()
    .wrap(IdentityMiddleware::default())
    .wrap(
        SessionMiddleware::new(
            CookieSessionStore::default(),
            get_secret_key()
        )
        
    ).app_data(Data::new(Mutex::new(Posts(vec![
        Post{
            id: "1".to_string(),
            published_at: Utc::now() - Duration::days(15),
            author: "123".to_string(),
            title: "first blog post".to_string(),
            content: "Proident ex officia dolor nulla eiusmod eiusmod do ullamco officia velit laboris. Aliqua commodo duis ut do occaecat exercitation. Magna commodo irure fugiat nisi consequat nulla culpa labore consequat nisi sint est pariatur deserunt. Elit cillum tempor reprehenderit mollit excepteur sint ea eu do et sint culpa.".to_string()
        },
        Post{
            id: "2".to_string(),
            published_at: Utc::now() - Duration::days(5),
            author: "123".to_string(),
            title: "hello world".to_string(),
            content: "Velit excepteur duis fugiat pariatur non esse veniam aute officia qui. Cupidatat tempor cillum culpa anim occaecat Lorem reprehenderit laboris id ut ut dolore. Ipsum qui sint do id eiusmod eiusmod dolore quis. Do pariatur mollit anim ullamco anim laboris proident sunt quis aliqua incididunt Lorem ullamco. Pariatur eiusmod magna ex ex nulla excepteur ex pariatur irure ad. Reprehenderit et esse sunt minim voluptate sint.".to_string()
        },
        Post{
            id: "3".to_string(),
            published_at: Utc::now() - Duration::days(2),
            author: "345".to_string(),
            title: "lastest update".to_string(),
            content: "Nisi cupidatat ex aliqua aliqua tempor esse enim fugiat deserunt voluptate et occaecat proident. Labore officia veniam ipsum officia velit. Anim voluptate quis deserunt veniam labore eu nulla ipsum consectetur. Dolor ad id in deserunt voluptate veniam sunt velit officia nulla. Esse labore minim voluptate aliqua sit adipisicing labore. Labore irure sunt officia nostrud ut id anim sint eu et duis deserunt.".to_string()
        },
        Post{
            id: "1".to_string(),
            published_at: Utc::now() - Duration::days(15),
            author: "123".to_string(),
            title: "first blog post".to_string(),
            content: "Proident ex officia dolor nulla eiusmod eiusmod do ullamco officia velit laboris. Aliqua commodo duis ut do occaecat exercitation. Magna commodo irure fugiat nisi consequat nulla culpa labore consequat nisi sint est pariatur deserunt. Elit cillum tempor reprehenderit mollit excepteur sint ea eu do et sint culpa.".to_string()
        },
        Post{
            id: "2".to_string(),
            published_at: Utc::now() - Duration::days(5),
            author: "123".to_string(),
            title: "hello world".to_string(),
            content: "Velit excepteur duis fugiat pariatur non esse veniam aute officia qui. Cupidatat tempor cillum culpa anim occaecat Lorem reprehenderit laboris id ut ut dolore. Ipsum qui sint do id eiusmod eiusmod dolore quis. Do pariatur mollit anim ullamco anim laboris proident sunt quis aliqua incididunt Lorem ullamco. Pariatur eiusmod magna ex ex nulla excepteur ex pariatur irure ad. Reprehenderit et esse sunt minim voluptate sint.".to_string()
        },
        Post{
            id: "3".to_string(),
            published_at: Utc::now() - Duration::days(2),
            author: "345".to_string(),
            title: "lastest update".to_string(),
            content: "Nisi cupidatat ex aliqua aliqua tempor esse enim fugiat deserunt voluptate et occaecat proident. Labore officia veniam ipsum officia velit. Anim voluptate quis deserunt veniam labore eu nulla ipsum consectetur. Dolor ad id in deserunt voluptate veniam sunt velit officia nulla. Esse labore minim voluptate aliqua sit adipisicing labore. Labore irure sunt officia nostrud ut id anim sint eu et duis deserunt.".to_string()
        },
        Post{
            id: "1".to_string(),
            published_at: Utc::now() - Duration::days(15),
            author: "123".to_string(),
            title: "first blog post".to_string(),
            content: "Proident ex officia dolor nulla eiusmod eiusmod do ullamco officia velit laboris. Aliqua commodo duis ut do occaecat exercitation. Magna commodo irure fugiat nisi consequat nulla culpa labore consequat nisi sint est pariatur deserunt. Elit cillum tempor reprehenderit mollit excepteur sint ea eu do et sint culpa.".to_string()
        },
        Post{
            id: "2".to_string(),
            published_at: Utc::now() - Duration::days(5),
            author: "123".to_string(),
            title: "hello world".to_string(),
            content: "Velit excepteur duis fugiat pariatur non esse veniam aute officia qui. Cupidatat tempor cillum culpa anim occaecat Lorem reprehenderit laboris id ut ut dolore. Ipsum qui sint do id eiusmod eiusmod dolore quis. Do pariatur mollit anim ullamco anim laboris proident sunt quis aliqua incididunt Lorem ullamco. Pariatur eiusmod magna ex ex nulla excepteur ex pariatur irure ad. Reprehenderit et esse sunt minim voluptate sint.".to_string()
        },
        Post{
            id: "3".to_string(),
            published_at: Utc::now() - Duration::days(2),
            author: "345".to_string(),
            title: "lastest update".to_string(),
            content: "Nisi cupidatat ex aliqua aliqua tempor esse enim fugiat deserunt voluptate et occaecat proident. Labore officia veniam ipsum officia velit. Anim voluptate quis deserunt veniam labore eu nulla ipsum consectetur. Dolor ad id in deserunt voluptate veniam sunt velit officia nulla. Esse labore minim voluptate aliqua sit adipisicing labore. Labore irure sunt officia nostrud ut id anim sint eu et duis deserunt.".to_string()
        },
        Post{
            id: "1".to_string(),
            published_at: Utc::now() - Duration::days(15),
            author: "123".to_string(),
            title: "first blog post".to_string(),
            content: "Proident ex officia dolor nulla eiusmod eiusmod do ullamco officia velit laboris. Aliqua commodo duis ut do occaecat exercitation. Magna commodo irure fugiat nisi consequat nulla culpa labore consequat nisi sint est pariatur deserunt. Elit cillum tempor reprehenderit mollit excepteur sint ea eu do et sint culpa.".to_string()
        },
        Post{
            id: "2".to_string(),
            published_at: Utc::now() - Duration::days(5),
            author: "123".to_string(),
            title: "hello world".to_string(),
            content: "Velit excepteur duis fugiat pariatur non esse veniam aute officia qui. Cupidatat tempor cillum culpa anim occaecat Lorem reprehenderit laboris id ut ut dolore. Ipsum qui sint do id eiusmod eiusmod dolore quis. Do pariatur mollit anim ullamco anim laboris proident sunt quis aliqua incididunt Lorem ullamco. Pariatur eiusmod magna ex ex nulla excepteur ex pariatur irure ad. Reprehenderit et esse sunt minim voluptate sint.".to_string()
        },
        Post{
            id: "3".to_string(),
            published_at: Utc::now() - Duration::days(2),
            author: "345".to_string(),
            title: "lastest update".to_string(),
            content: "Nisi cupidatat ex aliqua aliqua tempor esse enim fugiat deserunt voluptate et occaecat proident. Labore officia veniam ipsum officia velit. Anim voluptate quis deserunt veniam labore eu nulla ipsum consectetur. Dolor ad id in deserunt voluptate veniam sunt velit officia nulla. Esse labore minim voluptate aliqua sit adipisicing labore. Labore irure sunt officia nostrud ut id anim sint eu et duis deserunt.".to_string()
        },
        Post{
            id: "1".to_string(),
            published_at: Utc::now() - Duration::days(15),
            author: "123".to_string(),
            title: "first blog post".to_string(),
            content: "Proident ex officia dolor nulla eiusmod eiusmod do ullamco officia velit laboris. Aliqua commodo duis ut do occaecat exercitation. Magna commodo irure fugiat nisi consequat nulla culpa labore consequat nisi sint est pariatur deserunt. Elit cillum tempor reprehenderit mollit excepteur sint ea eu do et sint culpa.".to_string()
        },
        Post{
            id: "2".to_string(),
            published_at: Utc::now() - Duration::days(5),
            author: "123".to_string(),
            title: "hello world".to_string(),
            content: "Velit excepteur duis fugiat pariatur non esse veniam aute officia qui. Cupidatat tempor cillum culpa anim occaecat Lorem reprehenderit laboris id ut ut dolore. Ipsum qui sint do id eiusmod eiusmod dolore quis. Do pariatur mollit anim ullamco anim laboris proident sunt quis aliqua incididunt Lorem ullamco. Pariatur eiusmod magna ex ex nulla excepteur ex pariatur irure ad. Reprehenderit et esse sunt minim voluptate sint.".to_string()
        },
        Post{
            id: "3".to_string(),
            published_at: Utc::now() - Duration::days(2),
            author: "345".to_string(),
            title: "lastest update".to_string(),
            content: "Nisi cupidatat ex aliqua aliqua tempor esse enim fugiat deserunt voluptate et occaecat proident. Labore officia veniam ipsum officia velit. Anim voluptate quis deserunt veniam labore eu nulla ipsum consectetur. Dolor ad id in deserunt voluptate veniam sunt velit officia nulla. Esse labore minim voluptate aliqua sit adipisicing labore. Labore irure sunt officia nostrud ut id anim sint eu et duis deserunt.".to_string()
        },
    ])))).app_data(Data::new(HashMap::from([
        ("123".to_string(), Author{ id: "123".to_string(), name: "Damo".to_string()}), 
        ("345".to_string(), Author{ id: "345".to_string(), name: "Amanda".to_string()}), 
    ]))).service(index)
    
    .service(fs::Files::new("/static", "./resources/public").show_files_listing())
    .service(list_posts_endpoint)
    .service(edit_post_endpoint)
    .service(post_endpoint)
    .service(create_post_endpoint)
    .service(post_update_endpoint)
    .service(add_post_button_endpoint)
    .service(auth_status)
    .service(login)
    .service(logout)
)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
