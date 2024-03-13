use crate::Uri;
use crate::Url;
use std::collections::HashMap;

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
    println!("Received request for {}", &req.long_url);

    if !(req.long_url.starts_with("http://") || req.long_url.starts_with("https://")) {
        return Err(actix_web::error::ErrorBadRequest(
            "Invalid URL (must start with http(s)://)",
        ));
    }

    let mut cache = CACHE.write().unwrap();
    // Check if URL already exists
    if cache.set.contains(&req.long_url) {
        let short_url = cache.rev_map.get(&req.long_url).unwrap().to_owned();

        return Ok(web::Json(CreateShortUrlResponse { short_url }));
    }

    let short_url = generate_unique_token(&req.long_url, &cache.map);

    cache.set.insert(req.long_url.clone());
    cache.map.insert(short_url.clone(), req.long_url.clone());
    cache
        .rev_map
        .insert(req.long_url.clone(), short_url.clone());

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
