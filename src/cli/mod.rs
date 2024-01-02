pub struct CompilerArgs {
    pub run: bool,
    pub filename: String,
}

impl Default for CompilerArgs {
    fn default() -> Self {
        let run = true;
        let filename = "".into();
        Self {
            run,
            filename,
        }
    }
}