#[link(name = "compiled_code", kind = "static")]
extern "C" {

    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    #[link_name = "\x01start_here"]
    fn start_here() -> i64;
}

fn main() {
    let output = unsafe { start_here() };
    println!("{}", output);
}
