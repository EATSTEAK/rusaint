use std::{
    borrow::BorrowMut,
    io::{BufRead, Write},
    sync::Arc,
};

use cookie_store::serde::json::{load_all, save_incl_expired_and_nonpersistent};
use reqwest::{
    Client,
    cookie::{CookieStore, Jar},
    header::{COOKIE, HOST, HeaderValue, SET_COOKIE},
};
use reqwest_cookie_store::CookieStoreRwLock;
use url::Url;

use crate::{
    error::{RusaintError, SsuSsoError},
    utils::{DEFAULT_USER_AGENT, default_header},
    webdynpro::error::{ClientError, WebDynproError},
};

const SSU_USAINT_PORTAL_URL: &str = "https://saint.ssu.ac.kr/irj/portal";
const SSU_USAINT_SSO_URL: &str = "https://saint.ssu.ac.kr/webSSO/sso.jsp";
const SMARTID_LOGIN_URL: &str = "https://smartid.ssu.ac.kr/Symtra_sso/smln.asp";
const SMARTID_LOGIN_FORM_REQUEST_URL: &str = "https://smartid.ssu.ac.kr/Symtra_sso/smln_pcs.asp";

/// u-saint 로그인이 필요한 애플리케이션 사용 시 애플리케이션에 제공하는 세션
#[derive(Debug, Default)]
pub struct USaintSession(CookieStoreRwLock);

impl CookieStore for USaintSession {
    fn set_cookies(&self, cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, url: &Url) {
        self.0.set_cookies(cookie_headers, url)
    }

    fn cookies(&self, url: &Url) -> Option<HeaderValue> {
        self.0.cookies(url)
    }
}

impl USaintSession {
    /// 익명 세션을 반환합니다. 인증이 필요 없는 애플리케이션에서의 세션 동작과 동일합니다.
    pub fn anonymous() -> USaintSession {
        USaintSession(CookieStoreRwLock::default())
    }

    /// SSO 로그인 토큰과 학번으로 인증된 세션을 반환합니다.
    pub async fn with_token(id: &str, token: &str) -> Result<USaintSession, RusaintError> {
        let session_store = Self::anonymous();
        let client = Client::builder()
            .user_agent(DEFAULT_USER_AGENT)
            .build()
            .unwrap();
        // Manually include WAF cookies because of bug in reqwest::cookie::Jar
        let portal = client
            .get(SSU_USAINT_PORTAL_URL)
            .headers(default_header())
            .header(HOST, "saint.ssu.ac.kr".parse::<HeaderValue>().unwrap())
            .send()
            .await
            .map_err(|e| WebDynproError::from(ClientError::from(e)))?;
        let waf = portal.cookies().find(|cookie| cookie.name() == "WAF");

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

        if let Some(waf) = waf {
            let waf_cookie_str = format!("WAF={}; domain=saint.ssu.ac.kr; path=/;", waf.value());
            session_store
                .0
                .write()
                .unwrap()
                .parse(
                    &waf_cookie_str,
                    &Url::parse("https://saint.ssu.ac.kr").unwrap(),
                )
                .unwrap();
        } else {
            log::warn!("WAF cookie not found in portal response");
        }
        let token_cookie_str = format!("sToken={token}; domain=.ssu.ac.kr; path=/; secure");
        let req = client
            .get(format!("{SSU_USAINT_SSO_URL}?sToken={token}&sIdno={id}"))
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
            .build()
            .map_err(|e| WebDynproError::from(ClientError::from(e)))?;
        let res = client
            .execute(req)
            .await
            .map_err(|e| WebDynproError::from(ClientError::from(e)))?;
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
                .or(Err(ClientError::NoCookies(res.url().to_string())))
                .map_err(WebDynproError::from)?;
            if str.contains("MYSAPSSO2") {
                Ok(session_store)
            } else {
                Err(WebDynproError::from(ClientError::NoSuchCookie(
                    "MYSAPSSO2".to_string(),
                )))?
            }
        } else {
            Err(WebDynproError::from(ClientError::NoCookies(
                res.url().to_string(),
            )))?
        }
    }

    /// 학번과 비밀번호로 인증된 세션을 반환합니다.
    pub async fn with_password(id: &str, password: &str) -> Result<USaintSession, RusaintError> {
        let token = obtain_ssu_sso_token(id, password).await?;
        Self::with_token(id, &token).await
    }

    /// 현재 세션의 쿠키를 json 형식으로 저장합니다.
    pub fn save_to_json<W: Write>(&self, writer: &mut W) -> Result<(), RusaintError> {
        let store = self.0.read().unwrap();
        save_incl_expired_and_nonpersistent(&store, writer).map_err(|_| {
            WebDynproError::from(ClientError::NoCookies("Failed to save cookies".to_string()))
        })?;

        Ok(())
    }

    /// json 형식으로 저장된 쿠키를 읽어 세션을 생성합니다.
    pub fn from_json<R: BufRead>(reader: R) -> Result<USaintSession, RusaintError> {
        let store = load_all(reader).map_err(|_| {
            WebDynproError::from(ClientError::NoCookies("Failed to load cookies".to_string()))
        })?;
        let store = CookieStoreRwLock::new(store);
        Ok(USaintSession(store))
    }
}

/// 학번과 비밀번호를 이용해 SSO 토큰을 발급받습니다.
pub async fn obtain_ssu_sso_token(id: &str, password: &str) -> Result<String, SsuSsoError> {
    let jar: Arc<Jar> = Arc::new(Jar::default());
    let client = Client::builder()
        .cookie_provider(jar)
        .cookie_store(true)
        .user_agent(DEFAULT_USER_AGENT)
        .build()?;
    let body = client
        .get(SMARTID_LOGIN_URL)
        .headers(default_header())
        .send()
        .await?
        .text()
        .await?;
    let (in_tp_bit, rqst_caus_cd) = parse_login_form(&body)?;
    let params = [
        ("in_tp_bit", in_tp_bit.as_str()),
        ("rqst_caus_cd", rqst_caus_cd.as_str()),
        ("userid", id),
        ("pwd", password),
    ];
    let res = client
        .post(SMARTID_LOGIN_FORM_REQUEST_URL)
        .headers(default_header())
        .form(&params)
        .send()
        .await?;
    let cookie_token = {
        res.cookies()
            .find(|cookie| cookie.name() == "sToken" && !cookie.value().is_empty())
            .map(|cookie| cookie.value().to_string())
    };
    let message = if cookie_token.is_none() {
        let mut content = res.text().await?;
        let start = content.find("alert(\"").unwrap_or(0);
        let end = content.find("\");").unwrap_or(content.len());
        content.truncate(end);
        let message = content.split_off(start + 7);
        Some(message)
    } else {
        None
    };
    cookie_token.ok_or(SsuSsoError::CantFindToken(
        message.unwrap_or("Internal Error".to_string()),
    ))
}

fn parse_login_form(body: &str) -> Result<(String, String), SsuSsoError> {
    let document = scraper::Html::parse_document(body);
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
    Ok((in_tp_bit.to_owned(), rqst_caus_cd.to_owned()))
}
