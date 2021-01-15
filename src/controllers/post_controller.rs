use crate::database::DbCollection;
use crate::models::post::*;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use mongodb::Database;

#[get("/")]
async fn index(db: web::Data<Database>) -> impl Responder {
    let res = DbCollection::<Post>::new(db.as_ref()).index().await;
    web::Json(res)
}

#[get("/count/")]
async fn count(db: web::Data<Database>) -> impl Responder {
    let res = DbCollection::<Post>::new(db.as_ref()).count(None).await;
    match res {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(_) => HttpResponse::Ok().json(0),
    }
}

#[get("/{id}")]
async fn get(id: web::Path<String>, db: web::Data<Database>) -> impl Responder {
    let id_param = bson::oid::ObjectId::with_string(id.as_str());
    match id_param {
        Ok(id_val) => match DbCollection::<Post>::new(db.as_ref())
            .find_by_id(id_val)
            .await
        {
            Ok(res) => HttpResponse::Ok().json(res),
            Err(_) => HttpResponse::NotFound().json(()),
        },
        Err(_) => HttpResponse::BadRequest().json(()),
    }
}

#[post("/")]
async fn create(value: web::Json<Post>, db: web::Data<Database>) -> impl Responder {
    match DbCollection::<Post>::new(db.as_ref())
        .create(value.into_inner())
        .await
    {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::InternalServerError().json(()),
    }
}

#[delete("/{id}")]
async fn remove(id: web::Path<String>, db: web::Data<Database>) -> impl Responder {
    let id_param = bson::oid::ObjectId::with_string(id.as_str());
    match id_param {
        Ok(id_val) => {
            match DbCollection::<Post>::new(db.as_ref())
                .delete_by_id(id_val)
                .await
            {
                Ok(res) => HttpResponse::Ok().json(res),
                Err(_) => HttpResponse::NotFound().json(()),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(()),
    }
}

pub fn cfg_post_controller(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(count);
    cfg.service(get);
    cfg.service(create);
    cfg.service(remove);
}
