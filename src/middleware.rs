use actix_web::Result;

use crate::CACHE;

/// This function at the moment just updates the local cache for usage
/// It is called post processing the request as to avoid the overhead of writing
/// to disks in future
pub fn capture_usage(req_uri: &str) -> Result<()> {
    // Since this function is called after the processing
    // We can assume all the necessary checks are made, and hence we can
    // update the cache directly
    let mut wcache = CACHE.write().unwrap();

    wcache
        .usage_map
        .entry(req_uri.into())
        .and_modify(|count| *count += 1)
        .or_insert(1);

    println!(
        "updated usage stats for {} to {}",
        req_uri,
        wcache.usage_map.get(req_uri).unwrap()
    );
    Ok(())
}
