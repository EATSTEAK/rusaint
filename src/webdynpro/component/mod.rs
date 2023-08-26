mod button;

trait Component<'a> {}

trait Parseable<'a>: Component<'a> {
    type Output;

    fn parse() -> Self::Output;
}