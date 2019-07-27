use pest_derive::*;
use pest::Parser as _;

#[derive(Parser)]
#[grammar_inline = r#"
    number = @{ "x" }
"#]
struct Parser;

fn main() {
    let mut pairs = Parser::parse(Rule::number, "x").unwrap();
    let number = pairs.next().unwrap();
    dbg!(&number);

    let empty = number.into_inner();
    dbg!(&empty);

    empty.as_str();
}
