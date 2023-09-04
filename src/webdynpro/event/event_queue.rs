use std::{collections::LinkedList, ops::{Deref, DerefMut}};
use super::{WDEvent, EVENT_SPECTATOR};


pub struct WDEventQueue(LinkedList<WDEvent>);

impl Deref for WDEventQueue {
    type Target = LinkedList<WDEvent>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WDEventQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl WDEventQueue {

    pub fn new() -> WDEventQueue {
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

    pub fn add(&mut self, evt: WDEvent) {
        self.push_back(evt)
    }

    pub fn remove(&mut self) -> Option<WDEvent> {
        self.pop_front()
    }
}