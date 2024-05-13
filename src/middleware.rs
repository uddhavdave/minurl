use actix_web::http::StatusCode;
use lazy_static::lazy_static;
use prometheus::{opts, register_int_counter_vec, IntCounterVec};

lazy_static! {
    pub static ref HTTP_MINURL_REQUESTS_TOTAL: IntCounterVec = register_int_counter_vec!(
        opts!("http_minurl_requests", "HTTP MinURL requests"),
        &["status", "redirect_to"]
    )
    .expect("Can't create HTTP_REQUESTS_TOTAL metric");
}

/// Middleware function to capture minurl redirect requests for prometheus server
pub fn capture_redirection_usage(status: StatusCode, redirected_url: &str) {
    HTTP_MINURL_REQUESTS_TOTAL
        .with_label_values(&[status.as_str(), redirected_url])
        .inc();
}
