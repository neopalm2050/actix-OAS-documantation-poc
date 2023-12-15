use std::sync::RwLock;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    tings: RwLock<u32>,
}

/**
 * Output ting count
 */
#[get("/ting")]
async fn get_tings(data: web::Data<AppState>) -> impl Responder {
    let tings = match data.tings.read() {
        Ok(guard) => *guard,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    
    let response = format!("{}", tings);
    HttpResponse::Ok().body(response)
}

/**
 * Increment ting count
 */
#[post("/ting")]
async fn ting(data: web::Data<AppState>) -> impl Responder {
    let tings = match data.tings.write() {
        Ok(mut guard) => {*guard += 1; *guard},
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    
    let response = format!("{}", tings);
    HttpResponse::Ok().body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let initial_state = web::Data::new (AppState {
        tings: RwLock::new(0),
    });
    
    HttpServer::new(move || {
        App::new()
            .app_data(initial_state.clone())
            .service(get_tings)
            .service(ting)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}