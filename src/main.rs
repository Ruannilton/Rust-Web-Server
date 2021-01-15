mod controllers;
mod database;
mod models;

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use controllers::{contact_controller, post_controller, product_controller};
use dotenv::dotenv;

use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client, Database,
};
use std::env;

#[get("/hello")]
async fn hello_actx(db: web::Data<Database>) -> impl Responder {
    let names = db.list_collection_names(None).await;

    HttpResponse::Ok().body(format!("Hello World!\n {:?}", names))
}

async fn connect_to_mongo() -> Database {
    let database_url = env::var("MONGO_STR").expect("MONGO_STR not set in .env");
    let client_op =
        ClientOptions::parse_with_resolver_config(&database_url, ResolverConfig::cloudflare())
            .await
            .expect("Failed to create mongo client options");
    let client = Client::with_options(client_op).expect("Failed to create mongo client");
    client.database("meiasjamais")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let db = connect_to_mongo().await;

    match HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .wrap(middleware::Logger::default())
            .service(hello_actx)
            .service(web::scope("contact").configure(contact_controller::cfg_contact_controller))
            .service(web::scope("product").configure(product_controller::cfg_product_controller))
            .service(web::scope("post").configure(post_controller::cfg_post_controller))
    })
    .bind("127.0.0.1:8000")
    {
        Ok(server) => {
            println!("Server running on 127.0.0.1:8000");
            server.run().await
        }
        Err(e) => {
            eprintln!("Failed to bootstrap server");
            Err(e)
        }
    }
}
