use crate::CACHE;
use actix_web::web::Redirect;
use actix_web::{get, web};

#[get("/{id}")]
pub async fn redirect_req(path: web::Path<(String,)>) -> Result<Redirect, actix_web::Error> {
    let token = path.into_inner().0;

    println!("Redirect request for {}", token);

    if let Some(long_url) = CACHE.read().unwrap().map.get(&token) {
        return Ok(Redirect::to(long_url.clone()));
    }

    Err(actix_web::error::ErrorNotFound("Short Url does not exist"))
}
