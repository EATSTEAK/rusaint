use super::application::client::body::WDBody;

pub mod button;
pub mod client_inspector;
pub mod combo_box;
pub mod custom;
pub mod form;
pub mod loading_placeholder;
pub mod tab_strip;

trait Element<'a> {}

trait Parseable<'a>: Element<'a> {
    type Parser;

    fn parser(&'a self, body: &'a WDBody) -> Self::Parser;
}
