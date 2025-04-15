// use actix_web::{App, HttpServer};
// use actix_files as fs;

// //static
// #[tokio::main]

// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(fs::Files::new("/static", "./static")
//                 .show_files_listing())
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::{delete, get, patch, post};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct Item {
    pub id: Option<u64>,
    pub name: String,
    pub des: String,
}

pub struct AppState {
    pub items: Mutex<Vec<Item>>,
}

#[get("/api/items")]
pub async fn get_items(data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    HttpResponse::Ok().json(&*items)
}

#[get("/api/items/{id}")]
pub async fn get_item(path: web::Path<u64>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let items = data.items.lock().unwrap();

    if let Some(item) = items.iter().find(|i| i.id == Some(id)) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}

#[post("/api/items")]
pub async fn create_item(
    item: web::Json<Item>,
    data: web::Data<AppState>
) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    let mut new_item = item.into_inner();
    
    // Generate new ID
    let new_id = items.len() as u64 + 1;
    new_item.id = Some(new_id);
    
    items.push(new_item.clone());
    HttpResponse::Created().json(new_item)
}


pub fn config_api(cfg: &mut web::ServiceConfig) {
    cfg.service(get_items).service(get_item).service(create_item);
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        items: Mutex::new(vec![Item {
            id: Some(1),
            name: "Item".to_string(),
            des: "dwawdawd".to_string(),
        }]),
    });
    HttpServer::new(move || App::new().app_data(app_state.clone()).configure(config_api))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
