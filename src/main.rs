use actix_web::dev::Service;
use actix_web::App;
use actix_web::HttpServer;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

mod apis;
mod error;
mod middleware;

use apis::create;
use apis::metrics;
use apis::redirect_req;
use middleware::capture_usage;

use std::sync::RwLock;

pub type Url = String;
pub type Uri = String;
pub type Cache = Arc<RwLock<CacheInner>>;

#[derive(Default)]
pub struct CacheInner {
    /// Maps shortened URIs to their respective URLs
    map: HashMap<Uri, Url>,
    /// Keeps track of URLs which are shortened
    set: HashSet<Url>,
    /// Tracks usage of short Urls
    usage_map: HashMap<Url, u32>,
}

impl CacheInner {
    pub fn new() -> Self {
        CacheInner {
            map: HashMap::new(),
            set: HashSet::new(),
            usage_map: HashMap::new(),
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
    // let session = create_db_session()
    //     .unwrap_or_else(|e| {
    //         eprintln!("Failed to connect to the database: {}", e);
    //         process::exit(1);
    //     })
    //     .get_connection()
    //     .unwrap();
    //
    println!("Started listening on port 8080");
    HttpServer::new(|| {
        App::new()
            .service(create)
            .service(metrics)
            .service(redirect_req)
            // following is the middleware function which updates the stats for analytics
            // Ideally these changes are made in a different Transaction or are
            // pushed to an external service like apache kafka, but here we are
            // tracking it on memory
            .wrap_fn(|req, svc| {
                let fut = svc.call(req);
                async {
                    let res = fut.await?;
                    // Checks if response is redirect
                    if let Some(location) = res.headers().get("location") {
                        let url = location.to_str().map_err(|_| {
                            actix_web::error::ErrorInternalServerError("Invalid redirection url")
                        })?;
                        let _ = capture_usage(url);
                    }
                    Ok(res)
                }
            })
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
