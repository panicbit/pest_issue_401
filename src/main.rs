use pest_derive::*;
use pest::Parser as _;

#[derive(Parser)]
#[grammar_inline = r#"
    x = @{ "x" }
"#]
struct Parser;

fn main() {
    let mut pairs = Parser::parse(Rule::x, "x").unwrap();
    let x = pairs.next().unwrap();
    dbg!(&x);

    let empty = x.into_inner();
    dbg!(&empty);

    empty.as_str();
}
