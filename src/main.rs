#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::collections::HashMap;

#[allow(dead_code)]
mod vm {
    pub const REGISTER_COUNT: usize = 128;

    pub type AddressType = u8;

    #[derive(Clone, Copy, Debug)]
    pub enum Value {
        Nil,

        Number(f32),
        Boolean(bool),

        Address(AddressType)
    }

    #[derive(Clone, Copy, Debug)]
    #[repr(u8)]
    pub enum CommandKind {
        Dismiss,
        HaltVM,

        Save,
        Copy,

        Add,
        Subtract,
        Multiply,
        Divide,
        IntDivide,
        Modulo,
        Power,

        LogicalAnd,
        LogicalOr,
        LogicalNot,

        Equal,
        NotEqual,
        GreaterThan,
        GreaterThanOrEqual,
        LessThan,
        LessThanOrEqual,

        Compare,
        Jump,
        JumpToBeginning,
        Break,
    }
    
    pub struct Command {
        pub kind: CommandKind,
        pub args: Vec<Value>
    }

    pub struct Routine {
        pub closure_addr: AddressType,
        pub pc: usize
    }

    impl Routine {
        pub fn new(closure_addr: AddressType) -> Self {
            Self {
                closure_addr,
                pc: 0
            }
        }
    }
}

fn main() {
    use vm::*;

    let mut closures: HashMap<AddressType, Vec<Command>> = HashMap::new();

    let main = vec![
        Command {
            kind: CommandKind::Save,
            args: vec![
                Value::Address(0),
                Value::Number(4.)
            ]
        },
        Command {
            kind: CommandKind::Multiply,
            args: vec![
                Value::Address(0),
                Value::Number(4.)
            ]
        },
        Command {
            kind: CommandKind::Add,
            args: vec![
                Value::Address(0),
                Value::Number(4.)
            ]
        },
    ];

    closures.insert(0, main);

    let mut routines: Vec<Routine> = vec![
        Routine::new(0)
    ];

    let mut registers = [vm::Value::Nil; vm::REGISTER_COUNT];

    'vm_loop: while let Some(routine) = routines.last_mut() {
        let pc = &mut routine.pc;
        let closure = closures.get(&routine.closure_addr).expect("invalid routine's closure address");
        let command = if let Some(command) = closure.get(*pc) {
            command
        } else {
            // no more commands to execute in this routine
            routines.pop();
            continue 'vm_loop;
        };

        macro_rules! error {
            ($fmt:expr) => {
                eprintln!("GVM:{}:{:?}: {}", pc, command.kind, $fmt);
            };
        }

        macro_rules! proceed {
            () => {
                *pc += 1;
                continue 'vm_loop;
            };
        }

        macro_rules! command_arg {
            ($idx:expr, $variant:path, $err_msg:expr) => {
                if let Some($variant(value)) = command.args.get($idx) {
                    value
                } else {
                    error!($err_msg);
                    proceed!();
                }
            };
        }

        match command.kind {
            CommandKind::Dismiss => {}

            CommandKind::HaltVM => {
                break 'vm_loop;
            }

            CommandKind::Save => {
                let dest = command_arg!(0, vm::Value::Address, "dest is not an Address");
                let dest = *dest as usize;

                let value = command_arg!(1, vm::Value::Number, "value is not a Number");

                if dest >= vm::REGISTER_COUNT {
                    error!("dest is out of bounds");
                    proceed!();
                } else {
                    registers[dest] = vm::Value::Number(*value);
                }
            }

            CommandKind::Copy => {
                let r1 = command_arg!(0, vm::Value::Address, "dest is not an Address");
                let dest = *r1 as usize;

                if dest >= vm::REGISTER_COUNT {
                    error!("dest is out of bounds");
                    proceed!();
                }

                let r2 = command_arg!(1, vm::Value::Address, "src is not an Address");
                let src = *r2 as usize;

                if src >= vm::REGISTER_COUNT {
                    error!("src is out of bounds");
                    proceed!();
                }
                
                registers[dest] = registers[src];
            }

            CommandKind::Add |
            CommandKind::Subtract |
            CommandKind::Multiply |
            CommandKind::Divide |
            CommandKind::IntDivide |
            CommandKind::Modulo |
            CommandKind::Power |
            CommandKind::Equal |
            CommandKind::NotEqual |
            CommandKind::GreaterThan |
            CommandKind::GreaterThanOrEqual |
            CommandKind::LessThan |
            CommandKind::LessThanOrEqual => {
                let dest = command_arg!(0, vm::Value::Address, "dest is not an Address");
                let dest = *dest as usize;

                let right = command_arg!(1, vm::Value::Number, "value is not a Number");
                let right = *right;

                if dest >= vm::REGISTER_COUNT {
                    error!("dest is out of bounds");
                    proceed!();
                }
                
                let (dest, left) = if let vm::Value::Number(value) = registers[dest] {
                    (&mut registers[dest], value)
                } else {
                    error!("dest's value is not a Number");
                    proceed!();
                };

                *dest = match command.kind {
                    CommandKind::Add => vm::Value::Number(left + right),
                    CommandKind::Subtract => vm::Value::Number(left - right),
                    CommandKind::Multiply => vm::Value::Number(left * right),
                    CommandKind::Divide => vm::Value::Number(left / right),
                    CommandKind::IntDivide => vm::Value::Number((left / right).floor()),
                    CommandKind::Modulo => vm::Value::Number(left % right),
                    CommandKind::Power => vm::Value::Number(left.powf(right)),
                    CommandKind::Equal => vm::Value::Boolean(left == right),
                    CommandKind::NotEqual => vm::Value::Boolean(left != right),
                    CommandKind::GreaterThan => vm::Value::Boolean(left > right),
                    CommandKind::GreaterThanOrEqual => vm::Value::Boolean(left >= right),
                    CommandKind::LessThan => vm::Value::Boolean(left < right),
                    CommandKind::LessThanOrEqual => vm::Value::Boolean(left <= right),
                    _ => unreachable!()
                };
            }

            CommandKind::LogicalAnd |
            CommandKind::LogicalOr |
            CommandKind::LogicalNot => {
                let dest = command_arg!(0, vm::Value::Address, "dest is not an Address");
                let dest = *dest as usize;

                let right = command_arg!(1, vm::Value::Boolean, "value is not a Boolean");
                let right = *right;

                if dest >= vm::REGISTER_COUNT {
                    error!("dest is out of bounds");
                    proceed!();
                }
                
                let (dest, left) = if let vm::Value::Boolean(value) = registers[dest] {
                    (&mut registers[dest], value)
                } else {
                    error!("dest's value is not a Boolean");
                    proceed!();
                };

                *dest = match command.kind {
                    CommandKind::LogicalAnd => vm::Value::Boolean(left && right),
                    CommandKind::LogicalOr => vm::Value::Boolean(left || right),
                    CommandKind::LogicalNot => vm::Value::Boolean(!right),
                    _ => unreachable!()
                };
            }

            _ => todo!()
        }

        proceed!();
    }

    println!("register 0 = {:?}", registers[0]);
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
