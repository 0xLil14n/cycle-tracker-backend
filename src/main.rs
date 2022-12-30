use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod db;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/cycle/{user_id}")]
async fn cycle(path: web::Path<u32>)-> impl Responder {
    let user_id = path.into_inner();

    HttpResponse::Ok().body(format!("cycle: {}", user_id))
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
    
    
    db::migrate_up().await;
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(cycle)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
