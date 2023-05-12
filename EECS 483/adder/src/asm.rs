/*
 * This file contains the type of our assembly instructions, their
 * arguments, register etc.
 */
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Reg {
    Rax,
    Rsp,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemRef {
    pub reg: Reg,
    pub offset: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arg64 {
    Reg(Reg),
    Imm(i64),
    Mem(MemRef),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arg32 {
    Reg(Reg),
    Imm(i32),
    Mem(MemRef),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reg32 {
    Reg(Reg),
    Imm(i32),
}

// Possible forms of arguments to the mov instruction
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MovArgs {
    ToReg(Reg, Arg64),    // mov Reg, Arg
    ToMem(MemRef, Reg32), // mov [memloc], Reg
}

// Possible forms of arguments to most binary instructions: add, sub,
// mul, etc.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinArgs {
    ToReg(Reg, Arg32),
    ToMem(MemRef, Reg32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instr {
    Mov(MovArgs),
    Add(BinArgs),
    Sub(BinArgs),
    Ret,
}

fn reg_to_string(r: Reg) -> String {
    match r {
	Reg::Rax => String::from("rax"),
	Reg::Rsp => String::from("rsp"),
    }
}

fn imm32_to_string(i: i32) -> String {
    i.to_string()
}

fn imm64_to_string(i: i64) -> String {
    i.to_string()
}

fn mem_ref_to_string(m: MemRef) -> String {
    format!("[{} + {}]", reg_to_string(m.reg), m.offset)
}

fn reg32_to_string(r_or_i: Reg32) -> String {
    match r_or_i {
        Reg32::Reg(r) => reg_to_string(r),
        Reg32::Imm(i) => imm32_to_string(i),
    }
}

fn arg32_to_string(arg: Arg32) -> String {
    match arg {
        Arg32::Reg(r) => reg_to_string(r),
        Arg32::Imm(i) => imm32_to_string(i),
        Arg32::Mem(m) => mem_ref_to_string(m),
    }
}

fn arg64_to_string(arg: Arg64) -> String {
    match arg {
        Arg64::Reg(r) => reg_to_string(r),
        Arg64::Imm(i) => imm64_to_string(i),
        Arg64::Mem(m) => mem_ref_to_string(m),
    }
}

fn mov_args_to_string(args: MovArgs) -> String {
    match args {
        MovArgs::ToReg(r, arg) => {
            format!("{}, {}", reg_to_string(r), arg64_to_string(arg))
        }
        MovArgs::ToMem(mem, arg) => {
            format!("{}, {}", mem_ref_to_string(mem), reg32_to_string(arg))
        }
    }
}

fn bin_args_to_string(args: BinArgs) -> String {
    match args {
        BinArgs::ToReg(r, arg) => {
            format!("{}, {}", reg_to_string(r), arg32_to_string(arg))
        }
        BinArgs::ToMem(mem, arg) => {
            format!("{}, {}", mem_ref_to_string(mem), reg32_to_string(arg))
        }
    }
}

pub fn instr_to_string(i: &Instr) -> String {
    match i {
        Instr::Mov(args) => {
            format!("        mov {}", mov_args_to_string(*args))
        }
        Instr::Add(args) => {
            format!("        add {}", bin_args_to_string(*args))
        }
        Instr::Sub(args) => {
            format!("        sub {}", bin_args_to_string(*args))
        }
	Instr::Ret => {
	    format!("        ret")
	}
    }
}

pub fn instrs_to_string(is: &[Instr]) -> String {
    let mut buf = String::new();
    for i in is {
        buf.push_str(&instr_to_string(i));
        buf.push_str("\n");
    }
    buf
}
