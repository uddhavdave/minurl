use crate::{db::DbExecutor, error::ApiError, Uri, Url};
use chrono::Utc;
use std::collections::HashMap;

/// Includes CRUD APIS for managing shortened URLS
use super::api_models::CreateShortUrlRequest;
use super::api_models::CreateShortUrlResponse;
use super::common::generate_token;
use crate::CACHE;
use actix_web::{post, web, Result};

#[post("/create")]
pub async fn create(
    db: web::Data<DbExecutor>,
    req: web::Json<CreateShortUrlRequest>,
) -> Result<web::Json<CreateShortUrlResponse>> {
    if !(req.long_url.starts_with("http://") || req.long_url.starts_with("https://")) {
        return Err(actix_web::error::ErrorBadRequest(
            "Invalid URL (must start with http(s)://)",
        ));
    }

    // Check if URL already exists
    if let Ok(rows) = db
        .get_url_by_long_url(req.long_url.clone())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?
        .rows_typed::<(String, chrono::DateTime<Utc>, String)>()
    {
        if let Some(Ok((short_url, _, _))) = rows.last() {
            println!("Existing entry found");
            return Ok(web::Json(CreateShortUrlResponse { short_url }));
        }
    }

    let short_url = generate_unique_token(&req.long_url, &CACHE.read().unwrap().map);
    db.insert_to_url(req.long_url.clone(), short_url.clone(), Utc::now())
        .await
        .map_err(|e| ApiError::DatabaseConnFailed(e.to_string()))?;

    Ok(web::Json(CreateShortUrlResponse { short_url }))
}

/// This function ensures unique token generation in case of token clash by simply
/// shifting the characters of the base64 generated string by 1 character and
/// verifying it in the cache
pub fn generate_unique_token(url: &str, map: &HashMap<Uri, Url>) -> String {
    let mut shift = 0u32;

    let mut token = generate_token(url, shift);

    while map.contains_key(&token) {
        shift += 1;
        token = generate_token(url, shift)
    }

    println!("generated");
    token
}
