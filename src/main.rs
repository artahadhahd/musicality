mod parser;
use parser::{
    error::ParseResponse,
    parser::{MusicalValues, Parser, ParsingFunctions},
};

pub struct Compiler {
    pub instructions: Vec<MusicalValues>,
}

// extern "C" {
//     pub fn cool_shit() -> i32;
// }

impl Compiler {
    pub fn from<'a>(input: &'a str) -> Self {
        let instructions: Vec<MusicalValues> = vec![];
        let _parser = Parser::from(input);
        Self { instructions }
    }
}

fn main() -> Result<(), ParseResponse> {
    // let i = unsafe { cool_shit() };
    let mut parser = Parser::from(include_str!("../test.musical"));
    parser.parse()?;
    Ok(())
}
