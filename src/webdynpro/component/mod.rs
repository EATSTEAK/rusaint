pub mod button;
pub mod client_inspector;
pub mod loading_placeholder;
pub mod form;

trait Component<'a> {}

trait Parseable<'a>: Component<'a> {
    type Output;

    fn parse() -> Self::Output;
}