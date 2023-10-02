use super::{Event, EVENT_SPECTATOR};
use std::{
    collections::LinkedList,
    ops::{Deref, DerefMut},
};

pub(crate) struct EventQueue(LinkedList<Event>);

impl Deref for EventQueue {
    type Target = LinkedList<Event>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EventQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue(LinkedList::new())
    }

    pub fn serialize_and_clear(&mut self) -> String {
        let mut owned = "".to_owned();
        let events = &self.0;
        for (idx, event) in events.iter().enumerate() {
            owned.push_str(&event.serialize());
            if idx < events.len() - 1 {
                owned.push_str(EVENT_SPECTATOR);
            }
        }
        let _ = &self.clear();
        owned
    }

    pub fn add(&mut self, evt: Event) {
        self.push_back(evt)
    }

    #[allow(unused)]
    pub fn remove(&mut self) -> Option<Event> {
        self.pop_front()
    }
}
