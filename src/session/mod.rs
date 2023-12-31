use std::{borrow::BorrowMut, sync::Arc};

use reqwest::{
    cookie::{CookieStore, Jar},
    header::{HeaderValue, COOKIE, HOST, SET_COOKIE},
    Client,
};
use url::Url;

use crate::{
    error::{RusaintError, SsuSsoError},
    utils::{default_header, DEFAULT_USER_AGENT},
    webdynpro::error::{ClientError, WebDynproError},
};

const SSU_USAINT_PORTAL_URL: &str = "https://saint.ssu.ac.kr/irj/portal";
const SSU_USAINT_SSO_URL: &str = "https://saint.ssu.ac.kr/webSSO/sso.jsp";
const SMARTID_LOGIN_URL: &str = "https://smartid.ssu.ac.kr/Symtra_sso/smln.asp";
const SMARTID_LOGIN_FORM_REQUEST_URL: &str = "https://smartid.ssu.ac.kr/Symtra_sso/smln_pcs.asp";

/// u-saint 로그인이 필요한 애플리케이션 사용 시 애플리케이션에 제공하는 세션
#[derive(Debug, Default)]
pub struct USaintSession(Jar);

impl CookieStore for USaintSession {
    fn set_cookies(&self, cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, url: &url::Url) {
        self.0.set_cookies(cookie_headers, url)
    }

    fn cookies(&self, url: &url::Url) -> Option<HeaderValue> {
        self.0.cookies(url)
    }
}

impl USaintSession {
    /// 익명 세션을 반환합니다. 인증이 필요 없는 애플리케이션에서의 세션 동작과 동일합니다.
    pub fn anonymous() -> USaintSession {
        USaintSession(Jar::default())
    }

    /// SSO 로그인 토큰과 학번으로 인증된 세션을 반환합니다.
    pub async fn with_token(id: &str, token: &str) -> Result<USaintSession, ClientError> {
        let session_store = Self::anonymous();
        let client = reqwest::Client::builder()
            .user_agent(DEFAULT_USER_AGENT)
            .build()
            .unwrap();
        // Manually include WAF cookies because of bug in reqwest::cookie::Jar
        let portal = client
            .get(SSU_USAINT_PORTAL_URL)
            .headers(default_header())
            .header(HOST, "saint.ssu.ac.kr".parse::<HeaderValue>().unwrap())
            .send()
            .await?;
        let waf = portal
            .cookies()
            .find(|cookie| cookie.name() == "WAF")
            .ok_or(ClientError::NoSuchCookie("WAF".to_string()))?;
        let waf_cookie_str = format!("WAF={}; domain=saint.ssu.ac.kr; path=/;", waf.value());
        session_store.set_cookies(
            portal
                .headers()
                .iter()
                .filter_map(|header| {
                    if header.0 == SET_COOKIE {
                        Some(header.1)
                    } else {
                        None
                    }
                })
                .borrow_mut(),
            portal.url(),
        );
        session_store.0.add_cookie_str(
            &waf_cookie_str,
            &Url::parse("https://saint.ssu.ac.kr").unwrap(),
        );
        let token_cookie_str = format!("sToken={}; domain=.ssu.ac.kr; path=/; secure", token);
        let req = client
            .get(format!(
                "{}?sToken={}&sIdno={}",
                SSU_USAINT_SSO_URL, token, id
            ))
            .query(&[("sToken", token), ("sIdno", id)])
            .headers(default_header())
            .header(
                COOKIE,
                session_store
                    .cookies(&Url::parse("https://saint.ssu.ac.kr").unwrap())
                    .unwrap(),
            )
            .header(COOKIE, token_cookie_str.parse::<HeaderValue>().unwrap())
            .header(HOST, "saint.ssu.ac.kr".parse::<HeaderValue>().unwrap())
            .build()?;
        let res = client.execute(req).await?;
        let mut new_cookies = res.headers().iter().filter_map(|header| {
            if header.0 == SET_COOKIE {
                Some(header.1)
            } else {
                None
            }
        });
        session_store.set_cookies(&mut new_cookies, res.url());
        if let Some(sapsso_cookies) = session_store.cookies(res.url()) {
            let str = sapsso_cookies
                .to_str()
                .or(Err(ClientError::NoCookies(res.url().to_string())))?;
            if str.contains("MYSAPSSO2") {
                Ok(session_store)
            } else {
                Err(ClientError::NoSuchCookie("MYSAPSSO2".to_string()))?
            }
        } else {
            Err(ClientError::NoCookies(res.url().to_string()))?
        }
    }

    /// 학번과 비밀번호로 인증된 세션을 반환합니다.
    pub async fn with_password(id: &str, password: &str) -> Result<USaintSession, RusaintError> {
        let token = obtain_ssu_sso_token(id, password).await?;
        Ok(Self::with_token(id, &token)
            .await
            .or_else(|e| Err(WebDynproError::Client(e)))?)
    }

    /// 세션의 내부 [`reqwest::cookie::Jar`]의 레퍼런스를 반환합니다.
    pub fn jar(&self) -> &Jar {
        &self.0
    }
}

/// 학번과 비밀번호를 이용해 SSO 토큰을 발급받습니다.
pub async fn obtain_ssu_sso_token(id: &str, password: &str) -> Result<String, SsuSsoError> {
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
        .find(|cookie| cookie.name() == "sToken" && !cookie.value().is_empty())
        .ok_or(SsuSsoError::CantFindToken)?;
    Ok(cookie_token.value().to_string())
}

#[cfg(test)]
mod test {
    use crate::session::{obtain_ssu_sso_token, USaintSession};
    use dotenv::dotenv;

    #[tokio::test]
    async fn get_session() {
        dotenv().ok();
        let id = std::env::var("SSO_ID").unwrap();
        let password = std::env::var("SSO_PASSWORD").unwrap();
        let session = USaintSession::with_password(&id, &password).await.unwrap();
        println!("{:?}", session);
    }

    #[tokio::test]
    async fn test_obtain_sso_token() {
        dotenv().ok();
        let id = std::env::var("SSO_ID").unwrap();
        let password = std::env::var("SSO_PASSWORD").unwrap();
        let token = obtain_ssu_sso_token(&id, &password).await.unwrap();
        assert_ne!("", token);
    }
}
