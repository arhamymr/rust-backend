use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


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


async fn index_page() -> impl Responder {
    "welcome index page"
}

async fn hello_page() -> impl Responder {
    "<p style='font-weight:bold'>Hello page</p>"
}


#[actix_web::main]  
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello))
        .service(
            web::scope("/app")
            .route("/index.html", web::get().to(index_page))
            .route("/hello.html", web::get().to(hello_page))
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}