use base64::prelude::*;

/// This function output a token string which is 6 characters long, enough to account for
/// ~ 68 Billion possible Unique URLS
pub fn generate_token(input: &str, shift: u32) -> String {
    let hash = md5::compute(input).to_vec();

    let token_chars = BASE64_STANDARD.encode(hash);

    token_chars
        .chars()
        .skip(shift as usize)
        .take(6)
        .collect::<String>()
}

#[cfg(test)]
mod test {
    use crate::apis::common::generate_token;

    #[test]
    /// Check idempotency
    pub fn check_idempotent_results() {
        let url1 = "google.com";

        assert_eq!(generate_token(url1, 0), generate_token(url1, 0))
    }

    #[test]
    /// Simple case for cheking Clashing tokens
    pub fn check_clash() {
        let url1 = "google.com";
        let url2 = "google.com/user";

        assert_ne!(generate_token(url1, 0), generate_token(url2, 0))
    }
}
