use actix_web::{HttpResponse, Responder, web};
use kotoba_player::Mask;
use serde::{Serialize, Deserialize};

use super::AppState;

#[derive(Serialize, Deserialize)]
pub struct Text {
    pub text: String,
}

#[derive(Deserialize)]
pub struct MaskRequestBody {
    pub text: String,
    pub mask_token: String,
    pub char_by_char: bool,
}

pub async fn parrot((text, state): (web::Json<Text>, web::Data<AppState>)) -> impl Responder {
    let mut kotoba = state.kotoba_player.lock().unwrap();
    let body = Text{text: kotoba.parrot(&text.text)};
    HttpResponse::Ok().json(body)
}

pub async fn masquerade((req, state): (web::Json<MaskRequestBody>, web::Data<AppState>)) -> impl Responder {
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