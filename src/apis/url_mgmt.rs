/// Includes CRUD APIS for managing shortened URLS
use super::api_models::CreateShortUrlRequest;
use super::api_models::CreateShortUrlResponse;
use super::common::generate_token;
use crate::CACHE;
use actix_web::{post, web, Result};

#[post("/create")]
pub async fn create(
    req: web::Json<CreateShortUrlRequest>,
) -> Result<web::Json<CreateShortUrlResponse>> {
    // Check if URL already exists
    if CACHE.read().unwrap().set.contains(&req.long_url) {
        return Err(actix_web::error::ErrorBadRequest("Url already shortened"));
    }

    let short_url = generate_token(&req.long_url);

    let mut writer = CACHE.write().unwrap();
    writer.set.insert(req.long_url.clone());
    writer.map.insert(short_url.clone(), req.long_url.clone());

    Ok(web::Json(CreateShortUrlResponse { short_url }))
}
