use std::ops::{Deref, DerefMut};

pub(crate) struct WDBody<'a>(tl::VDom<'a>);

impl<'a> Deref for WDBody<'a> {
    type Target = tl::VDom<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for WDBody<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}