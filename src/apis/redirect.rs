use crate::db::DbExecutor;
use crate::error::ApiError;
use crate::CACHE;
use actix_web::web::Redirect;
use actix_web::{get, web};

#[get("/{id}")]
pub async fn redirect_req(
    db: web::Data<DbExecutor>,
    path: web::Path<(String,)>,
) -> Result<Redirect, actix_web::Error> {
    let token = path.into_inner().0;

    println!("Redirect request for {}", token);

    if let Some(long_url) = CACHE.read().unwrap().map.get(&token) {
        return Ok(Redirect::to(long_url.clone()));
    }

    // Insert shortened url to DB
    if let Ok(mut rows) = db
        .get_url_by_uri(token)
        .await
        .map_err(|e| ApiError::DbTxnFailure(e.to_string()))?
        .rows_typed::<(String, String, String)>()
    {
        if let Some(Ok((short_url, long_url, _))) = rows.next() {
            println!("Caching the mapping");
            let mut cache = CACHE.write().unwrap();
            cache.map.insert(short_url.clone(), long_url.clone());
            return Ok(Redirect::to(long_url.clone()));
        }
    };

    Err(actix_web::error::ErrorNotFound("Short Url does not exist"))
}
