use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use kotoba_player::KotobaPlayer;
use std::sync::Mutex;

mod app;
mod analysis;

pub struct AppState {
    pub kotoba_player: Mutex<KotobaPlayer>,
}

#[get("/")]
async fn get() -> impl Responder {
    HttpResponse::Ok().body("This is Kotoba Asobi API.")
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
            .service(web::resource("/parrot")
                .route(web::put().to(app::parrot))
            )
            .service(web::resource("/masquerade")
                .route(web::put().to(app::masquerade))
            )
            .service(web::resource("/analyze")
                .route(web::put().to(analysis::analyze))
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}