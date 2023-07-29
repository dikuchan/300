use url::Url;

pub fn is_valid_token(token: &str) -> bool {
    token.starts_with("y0")
}

pub fn is_valid_url(url: &str) -> bool {
    Url::parse(url).is_ok()
}
