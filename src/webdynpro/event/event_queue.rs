use std::{collections::LinkedList, ops::{Deref, DerefMut}};
use super::{WDEvent, EVENT_SPECTATOR};


pub struct WDEventQueue<'a>(LinkedList<WDEvent<'a>>);

impl<'a> Deref for WDEventQueue<'a> {
    type Target = LinkedList<WDEvent<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for WDEventQueue<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> WDEventQueue<'a> {

    pub fn new() -> WDEventQueue<'a> {
        WDEventQueue(LinkedList::new())
    }

    pub fn serialize_and_clear(&mut self) -> String {
        let mut owned = "".to_owned();
        let events = &self.0;
        for (idx, event) in events.iter().enumerate() {
            owned.push_str(&event.serialize());
            if idx < events.len()-1 { owned.push_str(EVENT_SPECTATOR); }
        }
        let _ = &self.clear();
        owned
    }

    pub fn add(&mut self, evt: WDEvent<'a>) {
        &self.push_back(evt);
    }

    pub fn remove(&mut self) -> Option<WDEvent<'a>> {
        self.pop_front()
    }
}