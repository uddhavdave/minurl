use actix_web::get;
use actix_web::HttpResponse;
use actix_web::Responder;

use actix_web::http::header;
use prometheus::TextEncoder;

// use crate::CACHE;

#[get("/metrics")]
pub async fn metrics() -> impl Responder {
    let encoder = TextEncoder::new();
    let mut response = String::new();

    encoder
        .encode_utf8(&prometheus::gather(), &mut response)
        .expect("Failed to encode metrics");

    HttpResponse::Ok()
        .insert_header(header::ContentType(mime::TEXT_PLAIN))
        .body(response)
}
