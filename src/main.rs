use actix_web::{get, App, HttpServer, HttpResponse};

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

#[get("/test")]
async fn index() -> HttpResponse {
    let a = Person{ name: String::from("hoge"), age:18 };
    HttpResponse::Ok().json(a)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8082")?
        .run()
        .await
}