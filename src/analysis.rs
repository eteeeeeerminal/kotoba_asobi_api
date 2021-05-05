use actix_web::{HttpResponse, Responder, web};
use serde::{Serialize};

use super::AppState;
use super::app::Text;

#[derive(Serialize)]
struct TokensBody {
    tokens: Vec<kotoba_player::tokenizer::Token>
}

/// morphological analysis
pub async fn analyze((text, state): (web::Json<Text>, web::Data<AppState>)) -> impl Responder {
    let mut kotoba = state.kotoba_player.lock().unwrap();
    let body = TokensBody{tokens: kotoba.tokenizer.tokenize(&text.text)};
    HttpResponse::Ok().json(body)
}