use actix_web::get;
use actix_web::HttpResponse;
use actix_web::Responder;

use crate::CACHE;

#[get("/metrics")]
pub async fn metrics() -> impl Responder {
    let rcache = CACHE.read().unwrap();

    let mut sorted_entries: Vec<(&String, &u32)> = rcache.usage_map.iter().collect();

    sorted_entries.sort_by(|a, b| b.1.cmp(a.1));
    let top_3_domain_names: Vec<(String, u32)> = sorted_entries
        .iter()
        .take(3)
        .cloned()
        .collect::<Vec<(&String, &u32)>>()
        .into_iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();

    HttpResponse::Ok().json(top_3_domain_names)
}
