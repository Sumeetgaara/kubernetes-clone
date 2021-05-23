use actix_web::{get,web, App, HttpResponse, HttpServer, Responder};
use mobc_redis::RedisConnectionManager;
use mobc_redis::redis;
use redis::AsyncCommands;
use serde::Deserialize;
type Pool = mobc::Pool<RedisConnectionManager>;


#[derive(Deserialize)]
struct Info {
    key: String,
}

#[get("/api/stopasync/{key}")]
async fn stop_async(pool: web::Data<Pool>,info: web::Path<Info>) -> impl Responder {
    let key = info.key.to_string();
    let mut conn = pool.get().await.unwrap();
    let servers : String = conn.get(&key).await.unwrap();
    let count = servers.parse::<i32>().unwrap()-1; //count == 0 edge case is not handled as code is for demo purpose.
    let _: () = conn.set(&key,count.to_string()).await.unwrap();
    let _ : () = conn.publish("workers", count).await.unwrap();
    HttpResponse::Ok().body("success")
}

#[get("/api/startasync/{key}")]
async fn start_async(pool: web::Data<Pool>,info: web::Path<Info>) -> impl Responder {
    let key = info.key.to_string();
    let mut conn = pool.get().await.unwrap();
    let servers : String = conn.get(&key).await.unwrap();
    let count = servers.parse::<i32>().unwrap()+1; //count == 0 edge case is not handled as code is for demo purpose.
    let _: () = conn.set(&key,count.to_string()).await.unwrap();
    HttpResponse::Ok().body("success")
}

#[actix_rt::main]
async fn main() {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap(); //hardcoded for demo; we can place this in separate file.
    let manager = RedisConnectionManager::new(client);
    let pool = Pool::builder().max_open(100).build(manager);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(start_async)
            .service(stop_async)
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
    .unwrap();
}