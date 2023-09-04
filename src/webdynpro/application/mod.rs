
use url::Url;

use self::client::{WDClient, WDClientError, body::WDBody};

use super::event::event_queue::WDEventQueue;


pub struct BasicWDApplication {
    base_url: Url,
    name: String,
    client: WDClient
}

impl BasicWDApplication {
    pub async fn new(base_url_str: &str, name: &str) -> Result<Self, WDClientError> {
        let base_url = Url::parse(base_url_str)?;
        let client = WDClient::new(&base_url, name).await?;
        Ok(Self::with_client(base_url, name, client)?)
    }

    pub fn with_client(base_url: Url, name: &str, client: WDClient) -> Result<Self, WDClientError> {
        Ok(BasicWDApplication {
            base_url,
            name: name.to_owned(),
            client,
        })
    }

    pub fn client_url(&self) -> String {
        let mut url = "".to_owned();
        url.push_str(&self.base_url.as_str());
        if !url.ends_with('/') { url.push_str("/"); }
        url.push_str(&self.name);
        url.push_str("?sap-wd-stableids=X#");
        url
    }

    pub fn event_queue(&mut self) -> &mut WDEventQueue {
        &mut self.client.event_queue
    }

    pub async fn send_event(&mut self) -> Result<(), WDClientError> {
        self.client.send_event(&self.base_url).await
    }

    pub fn body(&mut self) -> &WDBody {
        &self.client.body
    }
}

pub mod client;