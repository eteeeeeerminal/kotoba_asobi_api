use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, delete, get, post, put, web};
use kotoba_player::KotobaPlayer;
use serde::{Serialize, Deserialize};
use std::sync::Mutex;

struct AppState {
    kotoba_player: Mutex<KotobaPlayer>,
}

#[derive(Serialize, Deserialize)]
struct Text {
    text: String,
}

#[get("/")]
async fn get() -> impl Responder {
    HttpResponse::Ok().body("This is Kotoba Asobi API.")
}

#[post("/parrot")]
async fn parrot((text, state): (web::Json<Text>, web::Data<AppState>)) -> impl Responder {
    let mut kotoba = state.kotoba_player.lock().unwrap();
    let body = Text{text: kotoba.parrot(&text.text)};
    HttpResponse::Ok().json(body)
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