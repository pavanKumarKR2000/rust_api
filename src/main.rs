#[macro_use]
extern crate diesel;
extern crate dotenv;

mod schema;
mod models;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use serde::{Serialize, Deserialize};
use std::env;
use std::sync::Mutex;
use crate::models::Post;

#[derive(Debug, Serialize, Deserialize)]
struct PostData {
    details: serde_json::Value,
    id: i32,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn create_post(dynamic: web::Path<String>, payload: web::Json<PostData>, connection: web::Data<Mutex<PgConnection>>) -> impl Responder {
    let id = payload.id;
    let details = &payload.details;
    let name = dynamic.into_inner();

    let mut connection = connection.lock().expect("Failed to lock connection");

    let existing_post = schema::posts::table
        .filter(schema::posts::name.eq(&name).and(schema::posts::id.eq(&id)))
        .first::<Post>(&mut *connection)
        .optional();
match existing_post {
        Ok(Some(_)) => {
            HttpResponse::Conflict().body(format!("Conflict: Post with key '{}' already exists with the same ID {}", name,id))
        },
        Ok(None) => {
            let new_post = Post { id, details: details.clone(), name };

            match diesel::insert_into(schema::posts::table)
                .values(&new_post)
                .execute(&mut *connection)
            {
                Ok(_) => {
                    HttpResponse::Ok().json(new_post)
                },
                Err(err) => {
                    println!("Error inserting post: {:?}", err);
                    HttpResponse::InternalServerError().finish()
                }
            }
        },
        Err(err) => {
            println!("Error querying post: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn get_data(dynamic: web::Path<(String, i32)>, connection: web::Data<Mutex<PgConnection>>) -> impl Responder {
    let (name, id) = dynamic.into_inner();

    let mut connection = connection.lock().expect("Failed to lock connection");
    let result = schema::posts::table
        .filter(schema::posts::name.eq(&name).and(schema::posts::id.eq(&id)))
        .first::<Post>(&mut *connection);

    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::NotFound().body(format!("Post with name '{}' and ID {} not found", name, id)),
    }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = web::Data::new(Mutex::new(establish_connection()));

    HttpServer::new(move || {
        App::new()
            .app_data(connection.clone())
            .route("/", web::get().to(index))
            .route("/api/v1/{dynamic}", web::post().to(create_post))
            .route("/api/v1/{dynamic}/{id}", web::get().to(get_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}