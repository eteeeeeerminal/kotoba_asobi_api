use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, delete, get, post, put, web};
use kotoba_player::{KotobaPlayer, Mask};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;

struct AppState {
    kotoba_player: Mutex<KotobaPlayer>,
}

#[derive(Serialize, Deserialize)]
struct Text {
    text: String,
}

#[derive(Deserialize)]
struct MaskRequestBody {
    text: String,
    mask_token: String,
    char_by_char: bool,
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

#[post("/masquerade")]
async fn masquerade((req, state): (web::Json<MaskRequestBody>, web::Data<AppState>)) -> impl Responder {
    let mut kotoba = state.kotoba_player.lock().unwrap();

    let mask  = if req.char_by_char {
        let c:Vec<char> = req.mask_token.chars().collect();
        if c.len() == 1 {
            Mask::CharByChar(c[0])
        }else {
            return HttpResponse::BadRequest().body("should mask token is one character when char by char masking");
        }
    } else {
        Mask::WordByWord(req.mask_token.clone())
    };

    let body = Text{text: kotoba.masquerade(&req.text, mask)};
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
            .service(masquerade)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}