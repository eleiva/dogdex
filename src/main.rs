mod models;
mod schema;

use self::schema::dogs::dsl::*;
use actix_files::Files;
use actix_web::http::{self, Error};
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer};
use awmp::Parts;
use diesel::r2d2::{self, ConnectionManager};
use diesel::prelude::*;
use dotenv::dotenv;
use handlebars::{DirectorySourceOptions, Handlebars};
use models::{Dog, NewDog};
use serde::Serialize;
use std::collections::HashMap;
use std::env;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Serialize)]
struct IndexTemplateData {
    project_name: String,
    dogs: Vec<self::models::Dog>,
    parent: String,
}

#[derive(Serialize)]

struct ViewTemplateData {
    dog: self::models::Dog,
    parent: String,
}


async fn index(hb: web::Data<Handlebars<'_>>, pool: web::Data<DbPool>) -> HttpResponse {
    let dogs_data = web::block(move || dogs.limit(100).load::<Dog>(&mut pool.get().unwrap()))
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish());

    let data = IndexTemplateData {
        project_name: "Perrunos".to_string(),
        dogs: dogs_data.unwrap().unwrap(),
        parent: "layout".to_string(),
    };

    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

async fn add(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = IndexTemplateData {
        project_name: "Perrunos".to_string(),
        dogs: vec![],
        parent: "layout".to_string(),
    };

    let body = hb.render("add", &data).unwrap();
    HttpResponse::Ok().body(body)
}

async fn dog(hb: web::Data<Handlebars<'_>>, pool: web::Data<DbPool>, dog_id: web::Path<i32>) -> HttpResponse {

    let dog_data = web::block(move || {
        dogs.filter(id.eq(dog_id.into_inner()))
        .first::<Dog>(&mut pool.get().unwrap())
        })
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish());


    let data = ViewTemplateData{
        dog: dog_data.unwrap().expect("Reason"),
        parent: "layout".to_string(),
    };
    
    let body = hb.render("view", &data).unwrap();
    HttpResponse::Ok().body(body)
}

async fn add_dog_form(pool: web::Data<DbPool>, mut parts: Parts) -> Result<HttpResponse, Error> {
    let file_path = parts
        .files
        .take("image")
        .pop()
        .and_then(|f| f.persist_in("./static/image").ok())
        .unwrap_or_default();

    let text_fields: HashMap<_, _> = parts.texts.as_pairs().into_iter().collect();

    let new_dog = NewDog {
        name: text_fields.get("name").unwrap().to_string(),
        image_path: file_path.to_string_lossy().to_string(),
    };

    let _ = web::block(move || {
        diesel::insert_into(dogs)
            .values(&new_dog)
            .execute(&mut pool.get().unwrap())
    })
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish());

    Ok(HttpResponse::SeeOther()
        .insert_header((http::header::LOCATION, "/".to_string()))
        .finish())
  
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mut handlebars = Handlebars::new();

    handlebars
        .register_templates_directory(
            "./static",
            DirectorySourceOptions {
                tpl_extension: ".html".to_owned(),
                hidden: false,
                temporary: false,
            },
        )
        .unwrap();

    let handlebars_ref = web::Data::new(handlebars);

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(&database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.");

    println!("Listening on port 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .app_data(Data::new(pool.clone()))
            .service(Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index))
            .route("/add", web::get().to(add))
            .route("/add_dog_form", web::post().to(add_dog_form))
            .route("/dog/{id}", web::get().to(dog))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
