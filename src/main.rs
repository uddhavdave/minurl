use actix_web::App;
use actix_web::{web, HttpServer, Responder};
// use cdrs::cluster::GetConnection;
use error::ApiError;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

mod apis;
// mod db;
mod error;

use apis::create;
// use db::create_db_session;

use std::sync::RwLock;

pub type Url = String;
pub type Uri = String;
pub type Cache = Arc<RwLock<CacheInner>>;

pub struct CacheInner {
    /// Maps shortened URIs to their respective URLs
    map: HashMap<Uri, Url>,
    /// Keeps track of URLs which are shortened
    set: HashSet<Url>,
}

impl CacheInner {
    pub fn new() -> Self {
        CacheInner {
            map: HashMap::new(),
            set: HashSet::new(),
        }
    }
}

lazy_static! {
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

    // Initiate local cache to store url mappings on memory
    // This state will be shared by all the APIs inorder to perform validations
    let _ = CACHE;
    HttpServer::new(|| App::new().service(create))
        .bind(("127.0.0.1", 8800))?
        .run()
        .await;

    Ok(())
}
