mod parser;
use parser::{
    error::ParseResponse,
    parser::{MusicalValues, Parser, ParsingFunctions},
};
use std::error::Error;
use std::{collections::HashMap, fmt::Display, fs, process::exit};

pub struct Compiler {
    pub ast: Vec<MusicalValues>,
    variables: VariableType,
    cursor: usize,
    function_pointer: HashMap<String, usize>,
}

#[derive(Debug)]
pub struct VariableType {
    pub global: HashMap<String, f32>,   // modified once
    pub scoped: HashMap<String, usize>, // cleared once entering a scope.
}

impl VariableType {
    pub fn new() -> Self {
        let global = HashMap::new();
        // global.insert(&"bpm".to_string(), 60f32);
        // global.insert(&"pitch".to_string(), 440f32);
        Self {
            global,
            scoped: HashMap::new(),
        }
    }
}

impl<'a> Compiler {
    pub fn new(input: &'a str) -> Result<Self, CompilerError> {
        let mut parser = Parser::from(input);
        let mut ast: Vec<MusicalValues> = vec![];
        let mut has_failed = false;
        let mut function_pointer = HashMap::new();
        let mut p = 0;
        while parser.has_next() {
            let mut label_name = "".to_string();
            if has_failed {
                break;
            }
            match parser.next() {
                Ok(v) => {
                    match &v {
                        MusicalValues::Label(name) => {
                            label_name += &name;
                            function_pointer.insert(label_name, p);
                        }
                        _ => (),
                    };
                    ast.push(v);
                }
                Err(e) => {
                    use ParseResponse as E;
                    match e {
                        E::Done => break,
                        E::NotPossible => unreachable!(),
                        e @ _ => {
                            eprintln!("{e}{}", parser.get_err_line());
                            has_failed = true;
                        }
                    }
                }
            }
            p += 1;
        }
        if has_failed {
            return Err(CompilerError::Failed);
        }
        if !function_pointer.contains_key("main") {
            return Err(CompilerError::NoMain);
        }
        let variables = VariableType::new();
        Ok(Self {
            ast,
            variables,
            cursor: 0,
            function_pointer,
        })
    }

    fn load_global_variables(&mut self) {
        let first_label = self
            .function_pointer
            .values()
            .min()
            .unwrap_or(&0usize)
            .clone();
        while self.cursor < first_label as usize {
            let mut key = "".to_string();
            match &self.ast[self.cursor] {
                MusicalValues::Var(v) => {
                    key += &v.name;
                    self.variables.global.insert(key, v.value);
                }
                e @ _ => println!("Warning: ignoring instruction {e}"),
            }
            self.cursor += 1;
        }
    }

    fn try_increase_cursor(&mut self) -> bool {
        if self.ast.len() > self.cursor + 1 {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    fn interpret_pair(&mut self, pair: (String, String)) -> Result<(), CompilerError> {
        match (pair.0.to_lowercase().as_str(), pair.1) {
            ("goto", label) => {
                if let Some(pointer) = self.function_pointer.get(&label) {
                    let prev = self.cursor;
                    self.cursor = pointer + 1 as usize;
                    self.run_body()?;
                    self.cursor = prev;
                } else {
                    eprintln!("Warning: ignoring 'goto {label}': label doesn't exist");
                }
                Ok(())
            }
            ("inc", var) => {
                if let Some(val) = self.variables.scoped.get(&var) {
                    self.variables.scoped.insert(var, val + 1);
                } else if let Some(val) = self.variables.global.get(&var) {
                    self.variables.global.insert(var, *val + 1f32);
                } else {
                    eprintln!("Warning: ignoring 'inc {var}': variable does not exist");
                }
                Ok(())
            }
            ("dec", var) => {
                if let Some(val) = self.variables.scoped.get(&var) {
                    self.variables.scoped.insert(var, val - 1);
                } else if let Some(val) = self.variables.global.get(&var) {
                    self.variables.global.insert(var, *val - 1f32);
                } else {
                    eprintln!("Warning: ignoring 'inc {var}': variable does not exist");
                }
                Ok(())
            }
            ("dbg", var) => {
                if let Some(val) = self.variables.scoped.get(&var) {
                    println!("SCOPED {var}: {val}")
                } else if let Some(val) = self.variables.global.get(&var) {
                    println!("GLOBAL {var}: {val}");
                } else {
                    println!("VARIABLE {var} doesn't exist.");
                }
                Ok(())
            }
            _ => Err(CompilerError::NoFunc(pair.0)),
        }
    }

    fn run_body(&mut self) -> Result<(), CompilerError> {
        match &self.ast[self.cursor] {
            MusicalValues::Label(_) => {
                // self.try_increase_cursor();
                return Ok(());
            }
            MusicalValues::Pair(p) => self.interpret_pair(p.clone())?,
            MusicalValues::Chord(c) => println!("{c:?}"),
            _ => todo!(),
        }
        if !self.try_increase_cursor() {
            return Ok(());
        }
        self.run_body()
    }

    pub fn run(&mut self) -> Result<(), CompilerError> {
        // println!("{:#?}", self.ast);
        self.load_global_variables();
        // set instruction pointer to main function's start.
        self.cursor = self.function_pointer.get("main").unwrap().clone();
        if !self.try_increase_cursor() {
            // main function is empty
            return Ok(());
        }
        self.run_body()?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum CompilerError {
    Failed,
    NoMain,
    NoFunc(String),
}

impl Error for CompilerError {}

impl<'a> Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Failed => write!(f, "Failed to compile due to previous parsing error"),
            Self::NoMain => write!(f, "File contains no main function"),
            Self::NoFunc(s) => write!(f, "function '{s}' doesn't exist"),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let shit = fs::read_to_string("test.musical")?;
    let compiler = Compiler::new(&shit);
    if let Err(e) = compiler {
        eprintln!("{e}");
        exit(1);
    }
    let mut compiler = compiler.unwrap();
    if let Err(e) = compiler.run() {
        eprintln!("{e}");
    }
    Ok(())
}
