use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use ffftf::api::post::list_posts;

#[get("/")]
async fn hello() -> impl Responder {
    let instance = String::from("https://lemmy.world");
    let posts = match list_posts(instance).await {
        Ok(x) => x,
        Err(e) => {
            println!("ERROR: {}", e);
            ffftf::api::post::Posts { posts: Vec::new() }
        }
    };
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Federated Forums For The Fans - v0.1.0");
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("localhost", 8000))?
        .run()
        .await
}
