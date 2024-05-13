use crate::db::DbExecutor;
use crate::error::ApiError;
use crate::CACHE;
use actix_web::web::Redirect;
use actix_web::{get, web};
use chrono::Utc;

#[get("/{id}")]
pub async fn redirect_req(
    db: web::Data<DbExecutor>,
    path: web::Path<(String,)>,
) -> Result<Redirect, actix_web::Error> {
    let token = path.into_inner().0;

    if let Some(long_url) = CACHE.read().unwrap().map.get(&token) {
        println!("Redirect request to {} from cache", long_url.clone());
        return Ok(Redirect::to(long_url.clone()));
    }

    if let Ok(mut rows) = db
        .get_url_by_uri(token)
        .await
        .map_err(|e| ApiError::DbTxnFailure(e.to_string()))?
        .rows_typed::<(String, chrono::DateTime<Utc>, String)>()
    {
        if let Some(Ok((short_url, _timestamp, long_url))) = rows.next() {
            println!("Caching result");
            let mut cache = CACHE.write().unwrap();
            cache.map.insert(short_url.clone(), long_url.clone());
            return Ok(Redirect::to(long_url.clone()));
        }
        println!("No matching row in DB found");
    };

    Err(actix_web::error::ErrorNotFound("Short Url does not exist"))
}
