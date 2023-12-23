mod parser;
use parser::{ParseResponse, Parser, ParsingFunctions};

fn main() -> Result<(), ParseResponse> {
    let mut parser = Parser::from(include_str!("../test.musical"));
    parser.parse()?;
    Ok(())
}
