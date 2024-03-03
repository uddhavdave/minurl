use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateShortUrlRequest {
    pub long_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateShortUrlResponse {
    pub short_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteShortUrlRequest {
    pub long_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteShortUrlResponse {
    pub short_url: String,
}
