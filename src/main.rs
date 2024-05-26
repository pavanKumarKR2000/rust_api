#[macro_use] 
extern crate diesel;
extern crate dotenv;

mod schema;
use crate::schema::*;

//use schema::posts;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::{Queryable, Insertable};
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
struct Post {
    id: i32,
    details: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PostData {
    details: String,
    id: i32,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn create_post(dynamic: web::Path<String>, payload: web::Json<PostData>, connection: web::Data<Mutex<PgConnection>>) -> impl Responder {
    let id = payload.id;
    let details = &payload.details;
    let name = dynamic.into_inner();

    let connection = connection.lock().expect("Failed to lock connection");

    // Check if a post with the provided key and ID already exists
    let existing_post = posts::table
        .filter(posts::name.eq(&name).and(posts::id.eq(&id)))
        .first::<Post>(&*connection)
        .optional();

    match existing_post {
        Ok(Some(_)) => {
            // A post with the same key and ID already exists
            // Respond with an error message
            HttpResponse::Conflict().body(format!("Conflict: Post with key '{}' already exists with the same ID", name))
        },
        Ok(None) => {
            // No post with the provided key and ID exists
            // Create the new post
            let new_post = Post { id, details: details.to_string(), name };
           match diesel::insert_into(posts::table)
                .values(&new_post)
                .execute(&*connection)
            {
                Ok(_) => {
                    // Successfully inserted the post
                    // Respond with the newly created post
                    HttpResponse::Ok().json(new_post)
                },
                Err(err) => {
                    // Handle the error if insertion fails
                    println!("Error inserting post: {:?}", err);
                    HttpResponse::InternalServerError().finish()
                }
            }
        },
        Err(err) => {
            // Handle the error if the query fails
            println!("Error querying post: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
async fn get_data(dynamic: web::Path<(String, i32)>, connection: web::Data<Mutex<PgConnection>>) -> impl Responder {
    let (name, id) = dynamic.into_inner();

    let connection = connection.lock().expect("Failed to lock connection");
    let result = posts::table
        .filter(posts::name.eq(&name).and(posts::id.eq(&id)))
        .first::<Post>(&*connection);

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

