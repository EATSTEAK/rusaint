pub mod button;
pub mod client_inspector;
pub mod custom;
pub mod loading_placeholder;
pub mod form;
pub mod combo_box;
pub mod tab_strip;


trait Component<'a> {}

trait Parseable<'a>: Component<'a> {
    type Output;

    fn parse() -> Self::Output;
}