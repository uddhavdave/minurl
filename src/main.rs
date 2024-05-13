use actix_web::dev::Service;
use actix_web::web::Data;
use actix_web::App;
use actix_web::HttpServer;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Arc;

mod apis;
mod db;
mod error;
mod middleware;

use apis::create;
use apis::metrics;
use apis::redirect_req;
use db::DbExecutor;
use middleware::capture_redirection_usage;

use std::sync::RwLock;

pub type Url = String;
pub type Uri = String;
pub type Cache = Arc<RwLock<CacheInner>>;

#[derive(Default)]
pub struct CacheInner {
    /// Maps shortened URIs to their respective URLs
    map: HashMap<Uri, Url>,
}

impl CacheInner {
    pub fn new() -> Self {
        CacheInner {
            // Mapping ShortUrls -> LongUrls
            map: HashMap::new(),
        }
    }
}

lazy_static! {
    /// Initiate local cache to store url mappings on memory
    /// This state will be shared by all the APIs inorder to perform validations
    static ref CACHE: Cache = Arc::new(RwLock::new(CacheInner::new()));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("URL Shortener running.....");

    println!("Init Database.....");
    let db_handler = Data::new(DbExecutor::build().await.expect("connection failed"));

    println!("Started listening on port 8088");
    HttpServer::new(move || {
        App::new()
            .service(create)
            .service(metrics)
            .service(redirect_req)
            // following is the middleware function which updates the stats for analytics
            // where we are pushing metrics to prometheus
            .app_data(db_handler.clone())
            .wrap_fn(|req, svc| {
                let fut = svc.call(req);
                async {
                    let res = fut.await?;
                    // Checks if response is redirect
                    if let Some(location) = res.headers().get("location") {
                        let url = location.to_str().map_err(|_| {
                            actix_web::error::ErrorInternalServerError("Invalid redirection url")
                        })?;
                        capture_redirection_usage(res.status(), url);
                    }
                    Ok(res)
                }
            })
    })
    .bind(("0.0.0.0", 8088))?
    .run()
    .await
}
