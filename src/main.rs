mod parser;
use audio::AudioDevice;
use parser::{
    data::Chord,
    error::ParseResponse,
    parser::{MusicalValues, Parser, ParsingFunctions},
};
use std::error::Error;
use std::{collections::HashMap, fmt::Display, fs, process::exit};

mod audio;

fn play(chord: &Chord, _base: f32, _octave: f32) {
    for _note in chord.notes.iter() {
        // println!("playing note {note:?}, base = {base}, octave = {octave}");
    }
}

pub struct Compiler {
    pub ast: Vec<MusicalValues>,
    variables: VariableType,
    cursor: usize,
    function_pointer: HashMap<String, usize>,
    audio_device: Option<AudioDevice>,
}

#[derive(Debug)]
pub struct VariableType {
    pub global: HashMap<String, f32>,   // modified once
    pub scoped: HashMap<String, f32>, // cleared once entering a scope.
}

impl VariableType {
    pub fn new() -> Self {
        let global = HashMap::new();
        let scoped = HashMap::new();
        Self { global, scoped }
    }

    pub fn get_global(&self, var: &str) -> Result<f32, CompilerError> {
        if let Some(var) = self.global.get(var) {
            Ok(*var)
        } else {
            Err(CompilerError::GlobalPropertyMissing(var.into()))
        }
    }
}

impl<'a> Compiler {
    pub fn new(input: &'a str) -> Result<Self, CompilerError> {
        let audio_device = if AudioDevice::supports() {
            if cfg!(debug_assertions) {
                AudioDevice::enable_debug_mode();
            }
            let dev = AudioDevice::new();
            if let Err(e) = dev {
                eprintln!("{e}");
                None
            } else {
                Some(dev.unwrap())
            }
        } else {
            None
        };

        let mut parser = Parser::from(input);
        let mut ast = Vec::new();
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
            audio_device,
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
                    // self.variables.scoped.clear();
                    let last_scope_vars = self.variables.scoped.clone();
                    self.variables.scoped.clear();
                    let prevcursor = self.cursor;
                    self.cursor = *pointer;
                    self.try_increase_cursor();
                    self.run_body()?;
                    self.cursor = prevcursor;
                    self.variables.scoped = last_scope_vars;
                } else {
                    eprintln!("Warning: ignoring 'goto {label}': label doesn't exist");
                }
                Ok(())
            }
            ("inc", var) => {
                if let Some(val) = self.variables.scoped.get(&var) {
                    self.variables.scoped.insert(var, val + 1f32);
                } else if let Some(val) = self.variables.global.get(&var) {
                    self.variables.global.insert(var, *val + 1f32);
                } else {
                    eprintln!("Warning: ignoring 'inc {var}': variable does not exist");
                }
                Ok(())
            }
            ("dec", var) => {
                if let Some(val) = self.variables.scoped.get(&var) {
                    self.variables.scoped.insert(var, val - 1f32);
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
                return Ok(());
            }
            MusicalValues::Pair(p) => self.interpret_pair(p.clone())?,
            MusicalValues::Chord(chord) => {
                let base_pitch = self.variables.get_global("pitch")?;
                let octave = self.variables.get_global("octave")?;
                play(chord, base_pitch, octave);
            }
            MusicalValues::Var(v) => {
                self.variables.scoped.insert(v.name.clone(), v.value);
            }
        }
        if !self.try_increase_cursor() {
            return Ok(());
        }
        // dbg!(&self.variables);
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
    GlobalPropertyMissing(String),
}

impl Error for CompilerError {}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Failed => write!(f, "Failed to compile due to previous parsing error"),
            Self::NoMain => write!(f, "File contains no main function"),
            Self::NoFunc(s) => write!(f, "function '{s}' doesn't exist"),
            Self::GlobalPropertyMissing(s) => {
                write!(f, "Global property '{s}' is missing, but is required")
            }
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
        eprintln!("Error: {e}");
    }
    Ok(())
}
