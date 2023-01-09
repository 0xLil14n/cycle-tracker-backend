use std::time::{Instant, SystemTime};

use actix_web::{
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use db::to_pool;
use deadpool_postgres::{Client, Manager, Pool};
use serde::{Deserialize, Serialize};
mod db;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/cycle/{user_id}")]
async fn cycle(path: web::Path<u32>) -> impl Responder {
    let user_id = path.into_inner();

    HttpResponse::Ok().body(format!("cycle: {}", user_id))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[derive(Deserialize, Serialize)]

struct Event {
    user_id: i32,
    date: i64,
}


#[post("/cycle/period")]
async fn log_period(req_body: web::Json<Event>, pool: Data<Pool>) -> impl Responder {
    let user_id = req_body.user_id;
    let date = &req_body.date;
    let client: Client = pool.get().await.unwrap();

    let res = client
        .query(
            "INSERT INTO cycle (user_id, start_date) VALUES ($1, $2)",
            &[
                &user_id,
                &NaiveDateTime::from_timestamp_opt(*date, 0).expect("fuckkkk"),
            ],
        )
        .await;
        
    
    dbg!(&res);
    match &res {
        Ok(_) => HttpResponse::Ok().body("success"),
        Err(e) => HttpResponse::Ok().body(format!("error: {}", e))
    }
}

#[get("/test-db")]
async fn test_db(req_body: String, pool: Data<Pool>) -> impl Responder {
    let client: Client = pool.get().await.unwrap();
    let res: String = client
        .query("SELECT $1::TEXT", &[&"this is coming from the db"])
        .await
        .unwrap()
        .first()
        .unwrap()
        .get(0);

    HttpResponse::Ok().body(res)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let pool = to_pool();

    // db::migrate_up().await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(hello)
            .service(log_period)
            .service(echo)
            .service(cycle)
            .service(test_db)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
