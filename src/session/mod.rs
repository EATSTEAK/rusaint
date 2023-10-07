use std::{borrow::BorrowMut, ops::Deref};

use anyhow::Result;
use reqwest::{
    cookie::{CookieStore, Jar},
    header::{HeaderValue, COOKIE, HOST, SET_COOKIE},
};
use url::Url;

use crate::{
    utils::{default_header, obtain_ssu_sso_token, DEFAULT_USER_AGENT},
    webdynpro::error::ClientError,
};

const SSU_USAINT_PORTAL_URL: &str = "https://saint.ssu.ac.kr/irj/portal";
const SSU_USAINT_SSO_URL: &str = "https://saint.ssu.ac.kr/webSSO/sso.jsp";

#[derive(Debug, Default)]
pub struct USaintSession(Jar);

impl Deref for USaintSession {
    type Target = Jar;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CookieStore for USaintSession {
    fn set_cookies(&self, cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, url: &url::Url) {
        self.0.set_cookies(cookie_headers, url)
    }

    fn cookies(&self, url: &url::Url) -> Option<HeaderValue> {
        self.0.cookies(url)
    }
}

impl USaintSession {
    pub fn anonymous() -> USaintSession {
        USaintSession(Jar::default())
    }

    pub async fn with_token(id: &str, token: &str) -> Result<USaintSession> {
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
        session_store.add_cookie_str(
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
            let str = sapsso_cookies.to_str()?;
            if str.contains("MYSAPSSO2") {
                Ok(session_store)
            } else {
                Err(ClientError::NoSuchCookie("MYSAPSSO2".to_string()))?
            }
        } else {
            Err(ClientError::NoCookies(res.url().to_string()))?
        }
    }

    pub async fn with_password(id: &str, password: &str) -> Result<USaintSession> {
        let token = obtain_ssu_sso_token(id, password).await?;
        Self::with_token(id, &token).await
    }
}

#[cfg(test)]
mod test {
    use crate::session::USaintSession;
    use dotenv::dotenv;

    #[tokio::test]
    async fn get_session() {
        dotenv().ok();
        let id = std::env::var("SSO_ID").unwrap();
        let password = std::env::var("SSO_PASSWORD").unwrap();
        let session = USaintSession::with_password(&id, &password).await.unwrap();
        println!("{:?}", session);
    }
}
