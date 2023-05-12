use crate::asm::instrs_to_string;
use crate::asm::{Arg32, Arg64, BinArgs, Instr, MemRef, MovArgs, Reg, Reg32};
use crate::syntax::{Exp, Prim1};
use std::convert::TryInto;

// The possible error messages for the compiler
#[derive(Debug, PartialEq, Eq)]
pub enum CompileErr<Span> {
    // The location here is the Span of unbound variable
    UnboundVariable {
        unbound: String,
        location: Span,
    },

    // The Span here is the Span of the let-expression that has the duplicated bindings
    DuplicateBinding {
        duplicated_name: String,
        location: Span,
    },
}

// You may or may not find the following function useful.
//
// If you use it, make sure you have good reason why the .unwrap() will
// not fail.
fn usize_to_i32(x: usize) -> i32 {
    x.try_into().unwrap()
}

// Well-formedness checker
pub fn check_prog<Span>(e: &Exp<Span>) -> Result<(), CompileErr<Span>>
where Span: Clone // this means you can use the clone method on Span (which you will need to do)
{
    panic!("NYI: check_prog")
}

// Compile an expression to x86 code
// You can assume the the input expression has already passed the
// well-formedness checker.
fn compile_to_instrs<Ann>(e: &Exp<Ann>) -> Vec<Instr> {
    panic!("NYI: compile_to_instrs")
}

pub fn compile_to_string<Span>(e: &Exp<Span>) -> Result<String, CompileErr<Span>>
where Span: Clone
{
    let () = check_prog(e)?;
    let is = compile_to_instrs(&e);
    Ok(format!(
        "\
        section .text
        global start_here
start_here:
{}       
",
        instrs_to_string(&is)
    ))
}
