use crate::webdynpro::event::event_queue::WDEventQueue;
use self::client::WDClient;


struct WDApplication<'a> {
    base_url: &'a str,
    name: &'a str,
    client: WDClient,
    event_queue: WDEventQueue<'a>
}

trait Application<'a> {
    fn new() -> Self;

    fn with_client(client: WDClient) -> Self;
}


pub mod client;