use reqwest::header::{
    ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CACHE_CONTROL, CONNECTION, HeaderMap,
};

pub(crate) const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36";

pub(crate) fn default_header() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"
            .parse()
            .unwrap(),
    );
    headers.insert(ACCEPT_ENCODING, "deflate, br".parse().unwrap());
    headers.insert(ACCEPT_LANGUAGE, "ko,en;q=0.9,en-US;q=0.8".parse().unwrap());
    headers.insert(CACHE_CONTROL, "max-age=0".parse().unwrap());
    headers.insert(CONNECTION, "keep-alive".parse().unwrap());
    headers
}
