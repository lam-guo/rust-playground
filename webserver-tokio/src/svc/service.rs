use actix_web::web::{self};
use actix_web::{HttpResponse, Resource};
use serde::{Deserialize, Serialize};

pub fn init() -> Resource {
    web::resource("/wifi").route(web::post().to(set_wifi))
}

async fn set_wifi(payload: web::Json<WifiReq>) -> HttpResponse {
    return HttpResponse::Ok().json(payload);
}

#[derive(Serialize, Deserialize)]
pub struct WifiReq {
    pub ssid: String,
    pub passwd: String,
}
