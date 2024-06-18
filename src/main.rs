mod models;
mod schema;

use self::schema::dogs::dsl::*;
use actix_files::Files;
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer};
use awmp::Parts;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use handlebars::{DirectorySourceOptions, Handlebars};
use models::Dog;
use serde::Serialize;
use std::env;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Serialize)]
struct IndexTemplateData {
    project_name: String,
    dogs: Vec<self::models::Dog>,
    parent: String,
}

async fn index(hb: web::Data<Handlebars<'_>>, pool: web::Data<DbPool>) -> HttpResponse {
    let dogs_data = web::block(move || dogs.limit(100).load::<Dog>(&mut pool.get().unwrap()))
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish());

    let data = IndexTemplateData {
        project_name: "Perrunos".to_string(),
        dogs: dogs_data.unwrap().unwrap(),
        parent: "layout".to_string()
    };

    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

async fn add(hb: web::Data<Handlebars<'_>>) -> HttpResponse {

    let data = IndexTemplateData {
        project_name: "Perrunos".to_string(),
        dogs: vec![],
        parent: "layout".to_string()
    };

    let body = hb.render("add", &data).unwrap();
    HttpResponse::Ok().body(body)
}

async fn add_dog_form(pool: web::Data<DbPool>, parts: Parts) -> HttpResponse {
    
    dbg!(parts);
    HttpResponse::Ok().body({})
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
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
