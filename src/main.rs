use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tokio_postgres::{NoTls};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (client, a) = tokio_postgres::connect(
        "postgresql://dboperator:operatorpass123@localhost:5243/postgres",
        NoTls,
    ).await.expect("fk");
    
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS asdf_user (
            id              SERIAL PRIMARY KEY,
            username        VARCHAR UNIQUE NOT NULL,
            password        VARCHAR NOT NULL,
            email           VARCHAR UNIQUE NOT NULL
            )
    ",
    ).await.expect("fuck");
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
