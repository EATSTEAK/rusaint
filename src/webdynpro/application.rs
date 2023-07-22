use crate::webdynpro::{event_queue::WDEventQueue, model::SapSsrClient};

type WDBody<'a> = &'a str;

struct WDApplication<'a> {
    base_url: &'a str,
    id: &'a str,
    ssr_client: SapSsrClient<'a>,
    body: WDBody<'a>,
    event_queue: WDEventQueue<'a>
}

trait Application<'a> {
    
    fn init_anonymous() {
        
    }

    fn init_sso() {
        
    }
}



