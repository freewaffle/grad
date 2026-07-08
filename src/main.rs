#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::collections::HashMap;

#[allow(dead_code)]
mod vm {
    pub const REGISTER_COUNT: usize = 128;

    pub mod ptr {
        pub type Register = u8;
        pub type Instruction = u16;
    }

    #[derive(Clone, Copy, Debug)]
    pub enum Value {
        Nil,
        Number(f32),
        Boolean(bool),
    }

    #[repr(u8)]
    pub enum Command {
        Dismiss,
        HaltVM,

        /// Stores value `value` into register `dest`.
        Store {
            dest: ptr::Register,
            value: Value
        },

        /// Computes `dest + src` and stores result into `dest`.
        Add {
            dest: ptr::Register,
            src: ptr::Register
        },
        /// Computes `dest - src` and stores result into `dest`.
        Subtract {
            dest: ptr::Register,
            src: ptr::Register
        },
        /// Computes `dest * src` and stores result into `dest`.
        Multiply {
            dest: ptr::Register,
            src: ptr::Register
        },
        /// Computes `dest / src` and stores result into `dest`.
        Divide {
            dest: ptr::Register,
            src: ptr::Register
        },
        /// Computes `dest // src` and stores result into `dest`.
        IntDivide {
            dest: ptr::Register,
            src: ptr::Register
        },
        /// Computes `dest % src` and stores result into `dest`.
        Modulo {
            dest: ptr::Register,
            src: ptr::Register
        },
        /// Computes `dest ^ src` and stores result into `dest`.
        Power {
            dest: ptr::Register,
            src: ptr::Register
        },

        /// Computes `dest && src` and stores result into `dest`.
        LogicalAnd {
            dest: ptr::Register,
            src: ptr::Register
        },
        /// Computes `dest || src` and stores result into `dest`.
        LogicalOr {
            dest: ptr::Register,
            src: ptr::Register
        },
        /// Computes `!src` and stores result into `dest`.
        LogicalNot {
            dest: ptr::Register,
            src: ptr::Register
        },

        /// Computes `left == right` and stores result into `dest`.
        Equal {
            dest: ptr::Register,
            left: ptr::Register,
            right: ptr::Register
        },
        /// Computes `left != right` and stores result into `dest`.
        NotEqual {
            dest: ptr::Register,
            left: ptr::Register,
            right: ptr::Register
        },
        /// Computes `left > right` and stores result into `dest`.
        GreaterThan {
            dest: ptr::Register,
            left: ptr::Register,
            right: ptr::Register
        },
        /// Computes `left >= right` and stores result into `dest`.
        GreaterThanOrEqual {
            dest: ptr::Register,
            left: ptr::Register,
            right: ptr::Register
        },
        /// Computes `left < right` and stores result into `dest`.
        LessThan {
            dest: ptr::Register,
            left: ptr::Register,
            right: ptr::Register
        },
        /// Computes `left <= right` and stores result into `dest`.
        LessThanOrEqual {
            dest: ptr::Register,
            left: ptr::Register,
            right: ptr::Register
        },

        /// Skips the next instruction if `register` is false,
        /// otherwise does nothing.
        Compare {
            register: ptr::Register
        },
        /// Jumps to instruction `dest` in the current closure.
        Jump {
            dest: ptr::Instruction
        },
        /// Jumps to the beginning of the current closure.
        JumpToBeginning,
        /// Stops the current closure and returns to the previous one.
        Break,
    }

    impl Command {
        pub fn get_type_str(&self) -> &'static str {
            match self {
                Command::Dismiss => "Dismiss",
                Command::HaltVM => "HaltVM",
                Command::Store { .. } => "Store",
                Command::Add { .. } => "Add",
                Command::Subtract { .. } => "Subtract",
                Command::Multiply { .. } => "Multiply",
                Command::Divide { .. } => "Divide",
                Command::IntDivide { .. } => "IntDivide",
                Command::Modulo { .. } => "Modulo",
                Command::Power { .. } => "Power",
                Command::LogicalAnd { .. } => "LogicalAnd",
                Command::LogicalOr { .. } => "LogicalOr",
                Command::LogicalNot { .. } => "LogicalNot",
                Command::Equal { .. } => "Equal",
                Command::NotEqual { .. } => "NotEqual",
                Command::GreaterThan { .. } => "GreaterThan",
                Command::GreaterThanOrEqual { .. } => "GreaterThanOrEqual",
                Command::LessThan { .. } => "LessThan",
                Command::LessThanOrEqual { .. } => "LessThanOrEqual",
                Command::Compare { .. } => "Compare",
                Command::Jump { .. } => "Jump",
                Command::JumpToBeginning => "JumpToBeginning",
                Command::Break => "Break"
            }
        }
    }

    pub struct Routine {
        pub closure_addr: ptr::Register,
        pub pc: usize
    }

    impl Routine {
        pub fn new(closure_addr: ptr::Register) -> Self {
            Self {
                closure_addr,
                pc: 0
            }
        }
    }
}

fn main() {
    use vm::*;

    let mut closures: HashMap<ptr::Register, Vec<Command>> = HashMap::new();
    
    // simple loop:
    /*
    let main = vec![
        Command {
            kind: CommandKind::Store,
            args: vec![
                Value::Address(0),
                Value::Number(1000.)
            ]
        },
        Command {
            kind: CommandKind::Store,
            args: vec![
                Value::Address(1),
                Value::Number(0.)
            ]
        },
    ];

    let cycle = vec![
        Command {
            kind: CommandKind::Add,
            args: vec![
                Value::Address(1),
                Value::Number(1.)
            ]
        },
        Command {
            kind: CommandKind::GreaterThanOrEqual,
            args: vec![
                Value::Address(2),
                Value::Address(0),
                Value::Address(1)
            ]
        }
    ];
    */

    let main = vec![
        Command::Store { dest: 0, value: Value::Number(4.) },
        Command::Store { dest: 1, value: Value::Number(4.) },
        Command::Multiply { dest: 1, src: 0 },
        Command::Add { dest: 1, src: 0 }
    ];
            
    closures.insert(0, main);

    let mut routine_stack: Vec<Routine> = vec![
        Routine::new(0)
    ];

    let mut registers = [Value::Nil; REGISTER_COUNT];

    'vm_loop: while let Some(routine) = routine_stack.last_mut() {
        let pc = &mut routine.pc;
        let closure = closures.get(&routine.closure_addr).expect("invalid routine's closure address");
        let command = if let Some(command) = closure.get(*pc) {
            command
        } else {
            // no more commands left in this routine
            routine_stack.pop();
            continue 'vm_loop;
        };

        macro_rules! error {
            ($fmt:expr) => {
                eprintln!("GVM:{}:{:?}: {}", pc, command.get_type_str(), $fmt);
            };
        }

        macro_rules! proceed {
            () => {
                *pc += 1;
                continue 'vm_loop;
            };
        }

        macro_rules! fetch_value {
            ($addr:expr, $variant:path, $err_msg:expr) => {
                if let $variant(value) = registers[$addr] {
                    value
                } else {
                    error!($err_msg);
                    proceed!();
                }
            };
        }

        macro_rules! fetch_value_and_ptr {
            ($addr:expr, $variant:path, $err_msg:expr) => {
                if let $variant(value) = registers[$addr] {
                    (&mut registers[$addr], value)
                } else {
                    error!($err_msg);
                    proceed!();
                }
            };
        }

        macro_rules! check_address {
            ($addr:expr, $err_msg:expr) => {
                if $addr >= REGISTER_COUNT {
                    error!($err_msg);
                    proceed!();
                }
            };
        }

        match command {
            Command::Dismiss => {}

            Command::HaltVM => {
                break 'vm_loop;
            }

            Command::Store { dest, value } => {
                let dest = *dest as usize;
                check_address!(dest, "`dest` register is out of bounds");
                registers[dest] = *value;
            }

            Command::Add { dest, src } |
            Command::Subtract { dest, src } |
            Command::Multiply { dest, src } |
            Command::Divide { dest, src } |
            Command::IntDivide { dest, src } |
            Command::Modulo { dest, src } |
            Command::Power { dest, src } => {
                let src = *src as usize;
                check_address!(src, "`src` register is out of bounds");

                let value: f32 = if let Value::Number(value) = registers[src] {
                    value
                } else {
                    error!("specified register's value is not a Number");
                    proceed!();
                };

                let dest = *dest as usize;
                check_address!(dest, "`dest` register is out of bounds");
                
                let (dest, dest_value) = if let Value::Number(value) = registers[dest] {
                    (&mut registers[dest], value)
                } else {
                    error!("`dest` register's value is not a Number");
                    proceed!();
                };

                *dest = match command {
                    Command::Add { .. } => Value::Number(dest_value + value),
                    Command::Subtract { .. } => Value::Number(dest_value - value),
                    Command::Multiply { .. } => Value::Number(dest_value * value),
                    Command::Divide { .. } => Value::Number(dest_value / value),
                    Command::IntDivide { .. } => Value::Number((dest_value / value).floor()),
                    Command::Modulo { .. } => Value::Number(dest_value % value),
                    Command::Power { .. } => Value::Number(dest_value.powf(value)),
                    _ => unreachable!()
                };
            }

            Command::Equal { dest, left, right } |
            Command::NotEqual { dest, left, right } |
            Command::GreaterThan { dest, left, right } |
            Command::GreaterThanOrEqual { dest, left, right } |
            Command::LessThan { dest, left, right } |
            Command::LessThanOrEqual { dest, left, right } => {
                let left = *left as usize;
                check_address!(left, "`left` register is out of bounds");

                let left: f32 = if let Value::Number(value) = registers[left] {
                    value
                } else {
                    error!("`left` register's value is not a Number");
                    proceed!();
                };

                let right = *right as usize;
                check_address!(right, "`right` register is out of bounds");

                let right: f32 = if let Value::Number(value) = registers[right] {
                    value
                } else {
                    error!("`right` register's value is not a Number");
                    proceed!();
                };

                let dest = *dest as usize;
                check_address!(dest, "`dest` register is out of bounds");
                let dest = &mut registers[dest];

                *dest = match command {
                    Command::Equal { .. } => Value::Boolean(left == right),
                    Command::NotEqual { .. } => Value::Boolean(left != right),
                    Command::GreaterThan { .. } => Value::Boolean(left > right),
                    Command::GreaterThanOrEqual { .. } => Value::Boolean(left >= right),
                    Command::LessThan { .. } => Value::Boolean(left < right),
                    Command::LessThanOrEqual { .. } => Value::Boolean(left <= right),
                    _ => unreachable!()
                };
            }

            Command::LogicalAnd { dest, src } |
            Command::LogicalOr { dest, src } => {
                let src = *src as usize;
                check_address!(src, "`src` register is out of bounds");
                let src = fetch_value!(src, Value::Boolean, "`src` register's value is not a Boolean");

                let dest = *dest as usize;
                check_address!(dest, "`dest` register is out of bounds");
                let (dest, dest_value) = fetch_value_and_ptr!(dest, Value::Boolean, "`dest` register's value is not a Boolean");

                *dest = match command {
                    Command::LogicalAnd { .. } => Value::Boolean(dest_value && src),
                    Command::LogicalOr { .. } => Value::Boolean(dest_value || src),
                    _ => unreachable!()
                };
            }

            Command::LogicalNot { dest, src } => {
                let src = *src as usize;
                check_address!(src, "`src` register is out of bounds");
                let src = fetch_value!(src, Value::Boolean, "`src` register's value is not a Boolean");

                let dest = *dest as usize;
                check_address!(dest, "`dest` register is out of bounds");
                
                registers[dest] = Value::Boolean(!src);
            }

            _ => todo!()
        }

        proceed!();
    }

    println!("register 1 = {:?}", registers[1]);
}

/*

/* ---- tokenizer ---- */

let input = "add &var, 6 * 4";

let allowed_short_symbols = [',', '&', '*'];

let mut chars = input.chars().peekable();
for c in chars {
    if c.is_alphabetic() {
        let mut ident = String::new();
        loop {
            
        }
    }
}

*/
