mod domain;
mod posts;

use actix_files as fs;
use std::{collections::HashMap, sync::Mutex};

use actix_files::NamedFile;
use actix_web::{
    error::{self, InternalError},
    get,
    http::StatusCode,
    web::{self, Data},
    App, HttpServer, Responder,
};
use askama::Template;
use chrono::{Duration, Utc};
use domain::model::{
    author::Author,
    posts::{Post, Posts},
};
use posts::{edit_post_endpoint, list_posts_endpoint, post_endpoint, post_update_endpoint, create_post_endpoint};

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("index.html")?)
}

#[get("/hello/{name}")]
async fn hello(
    name: web::Path<String>,
) -> std::result::Result<impl Responder, InternalError<String>> {
    let hello = HelloTemplate { name: &name };
    hello
        .render()
        .map_err(|err| InternalError::new(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR))
}

pub type Authors = HashMap<String, Author>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().app_data(Data::new(Mutex::new(Posts(vec![
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
    ]))).service(index).service(hello).service(fs::Files::new("/static", "./resources/public").show_files_listing()).service(list_posts_endpoint).service(edit_post_endpoint).service(post_endpoint).service(create_post_endpoint).service(post_update_endpoint))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
