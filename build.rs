static FILE: &str = "src/sysaudio.cpp";

fn main() {
    println!("cargo:rerun-if-changed={FILE}");
    cc::Build::new()
        .cpp(true)
        .file(FILE)
        // .ar_flag("-lole32")
        .compile("audio");
    // panic!("{:?}", cc::Build::new().get_compiler());
}
