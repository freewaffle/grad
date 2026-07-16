#![allow(dead_code)]

pub mod ptr {
    pub type Register = u8;
    pub type Constant = u8;
    pub type Instruction = u8;
    pub type Closure = u8;
}

#[repr(u8)]
pub enum Instruction {
    NoOp,

    HaltVM,

    Move {
        to: ptr::Register,
        from: ptr::Register
    },
    LoadConstant {
        register: ptr::Register,
        constant: ptr::Constant,
    },
    LoadNil {
        from: ptr::Register,
        length: u16 /* opcode and `from` took two bytes from 32, so 16 left */
    },

    GlobalMove {
        to: ptr::Register,
        from: ptr::Register
    },
    GlobalLoadConstant {
        register: ptr::Register,
        constant: ptr::Constant,
    },
    GlobalLoadNil {
        from: ptr::Register,
        length: u16 /* opcode and `from` took two bytes from 32, so 16 left */
    },

    Add {
        r0: ptr::Register,
        r1: ptr::Register,
        r2: ptr::Register,
    },
    Sub {
        r0: ptr::Register,
        r1: ptr::Register,
        r2: ptr::Register,
    },
    Mul {
        r0: ptr::Register,
        r1: ptr::Register,
        r2: ptr::Register,
    },
    Div {
        r0: ptr::Register,
        r1: ptr::Register,
        r2: ptr::Register,
    },
    IDiv {
        r0: ptr::Register,
        r1: ptr::Register,
        r2: ptr::Register,
    },
    Mod {
        r0: ptr::Register,
        r1: ptr::Register,
        r2: ptr::Register,
    },
    Pow {
        r0: ptr::Register,
        r1: ptr::Register,
        r2: ptr::Register,
    },
    UnM {
        r0: ptr::Register,
        r1: ptr::Register,
    },
    Not {
        r0: ptr::Register,
        r1: ptr::Register,
    },

    Eq {
        r0: ptr::Register,
        r1: ptr::Register,
        r2: ptr::Register,
    },
    NEq {
        r0: ptr::Register,
        r1: ptr::Register,
        r2: ptr::Register,
    },
    Mt {
        r0: ptr::Register,
        r1: ptr::Register,
        r2: ptr::Register,
    },
    MtEq {
        r0: ptr::Register,
        r1: ptr::Register,
        r2: ptr::Register,
    },
    Lt {
        r0: ptr::Register,
        r1: ptr::Register,
        r2: ptr::Register,
    },
    LtEq {
        r0: ptr::Register,
        r1: ptr::Register,
        r2: ptr::Register,
    },

    Jump {
        to: u32,
    },
    Call {
        closure: ptr::Closure,
        head: ptr::Register
    },
    CallExt {
        func: ptr::Closure,
        head: ptr::Register
    },
    Return
}

impl Instruction {
    #[inline]
    pub fn get_type_str(&self) -> &'static str {
        use Instruction::*;
        match self {
            NoOp => "NoOp",
            HaltVM => "HaltVM",

            Move { .. } => "Move",
            LoadConstant { .. } => "Move",
            LoadNil { .. } => "Move",

            GlobalMove { .. } => "Move",
            GlobalLoadConstant { .. } => "Move",
            GlobalLoadNil { .. } => "Move",

            Add { .. } => "Add",
            Sub { .. } => "Sub",
            Mul { .. } => "Mul",
            Div { .. } => "Div",
            IDiv { .. } => "IDiv",
            Mod { .. } => "Mod",
            Pow { .. } => "Pow",
            UnM { .. } => "UnM",
            Not { .. } => "Not",

            Eq { .. } => "Eq",
            NEq { .. } => "NEq",
            Mt { .. } => "Mt",
            MtEq { .. } => "MtEq",
            Lt { .. } => "Lt",
            LtEq { .. } => "LtEq",

            Jump { .. } => "Jump",
            Call { .. } => "Call",
            CallExt { .. } => "CallExt",
            Return => "Return",
        }
    }
}
