use actix_web::{web, get, post, put, delete, App, HttpResponse, HttpServer, Responder};
use kotoba_player::KotobaPlayer;
use std::sync::Mutex;

struct AppState {
    kotoba_player: Mutex<KotobaPlayer>,
}

#[get("/")]
async fn get() -> impl Responder {
    HttpResponse::Ok().body("get ok")
}

#[get("/parrot")]
async fn parrot(data: web::Data<AppState>) -> impl Responder {
    let mut kotoba = data.kotoba_player.lock().unwrap();
    HttpResponse::Ok().body(kotoba.parrot("そうきちゃんは可愛いです"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState{
        kotoba_player: Mutex::new(KotobaPlayer::new("/app/dic")),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(get)
            .service(parrot)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}