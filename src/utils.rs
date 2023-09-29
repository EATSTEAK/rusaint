use std::sync::Arc;

use anyhow::Result;
use reqwest::{
    cookie::Jar,
    header::{HeaderMap, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CACHE_CONTROL, CONNECTION},
    Client,
};
use thiserror::Error;

pub const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36";
const SMARTID_LOGIN_URL: &str = "https://smartid.ssu.ac.kr/Symtra_sso/smln.asp";
const SMARTID_LOGIN_FORM_REQUEST_URL: &str = "https://smartid.ssu.ac.kr/Symtra_sso/smln_pcs.asp";

pub fn default_header() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"
            .parse()
            .unwrap(),
    );
    headers.insert(ACCEPT_ENCODING, "gzip, deflate, br".parse().unwrap());
    headers.insert(ACCEPT_LANGUAGE, "ko,en;q=0.9,en-US;q=0.8".parse().unwrap());
    headers.insert(CACHE_CONTROL, "max-age=0".parse().unwrap());
    headers.insert(CONNECTION, "keep-alive".parse().unwrap());
    headers
}
#[derive(Error, Debug)]
pub enum SsuSsoError {
    #[error("Can't load form data from page, is page changed?")]
    CantLoadForm,
    #[error("Token is not included in response.")]
    CantFindToken,
}

pub async fn obtain_ssu_sso_token(id: &str, password: &str) -> Result<String> {
    let jar: Arc<Jar> = Arc::new(Jar::default());
    let client = Client::builder()
        .cookie_provider(jar)
        .cookie_store(true)
        .user_agent(DEFAULT_USER_AGENT)
        .build()
        .unwrap();
    let body = client
        .get(SMARTID_LOGIN_URL)
        .headers(default_header())
        .send()
        .await?
        .text()
        .await?;
    let document = scraper::Html::parse_document(&body);
    let in_tp_bit_selector = scraper::Selector::parse(r#"input[name="in_tp_bit"]"#).unwrap();
    let rqst_caus_cd_selector = scraper::Selector::parse(r#"input[name="rqst_caus_cd"]"#).unwrap();
    let in_tp_bit = document
        .select(&in_tp_bit_selector)
        .next()
        .ok_or(SsuSsoError::CantLoadForm)?
        .value()
        .attr("value")
        .ok_or(SsuSsoError::CantLoadForm)?;
    let rqst_caus_cd = document
        .select(&rqst_caus_cd_selector)
        .next()
        .ok_or(SsuSsoError::CantLoadForm)?
        .value()
        .attr("value")
        .ok_or(SsuSsoError::CantLoadForm)?;
    let params = [
        ("in_tp_bit", in_tp_bit),
        ("rqst_caus_cd", rqst_caus_cd),
        ("userid", id),
        ("pwd", password),
    ];
    let res = client
        .post(SMARTID_LOGIN_FORM_REQUEST_URL)
        .headers(default_header())
        .form(&params)
        .send()
        .await?;
    let cookie_token = res
        .cookies()
        .find(|cookie| cookie.name() == "sToken")
        .ok_or(SsuSsoError::CantFindToken)?;
    Ok(cookie_token.value().to_string())
}

#[cfg(test)]
mod test {
    use crate::utils::obtain_ssu_sso_token;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_obtain_sso_token() {
        dotenv().ok();
        let id = std::env::var("SSO_ID").unwrap();
        let password = std::env::var("SSO_PASSWORD").unwrap();
        let token = obtain_ssu_sso_token(&id, &password).await.unwrap();
        assert_ne!("", token);
    }
}
