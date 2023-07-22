use self::body::WDBody;

pub struct WDClient<'a> {
    client: reqwest::Client,
    ssr_client: SapSsrClient<'a>,
    body: WDBody<'a>
}

pub struct SapSsrClient<'a> {
    action: &'a str,
    charset: &'a str,
    wd_secure_id: &'a str,
    app_name: &'a str,
    use_beacon: bool
}

pub mod body;