use std::{collections::LinkedList, ops::{Deref, DerefMut}};
use super::WDEvent;


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
    pub fn serialize_and_clear(&mut self) -> String {
        todo!("Implement Serialization");
    }

    pub fn add(&mut self, evt: WDEvent<'a>) {
        &self.push_back(evt);
    }

    pub fn remove(&mut self) -> Option<WDEvent<'a>> {
        self.pop_front()
    }
}