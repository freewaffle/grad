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

    let cycle = vec![
        Command {
            kind: CommandKind::Add,
            args: vec![
                Value::Address(0),
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

    let main = vec![
        Command {
            kind: CommandKind::Save,
            args: vec![
                Value::Address(1),
                Value::Number(1000.)
            ]
        },
    ];

    closures.insert(0, main);

    let mut routines: Vec<Routine> = vec![
        Routine::new(0)
    ];

    let mut registers = [Value::Nil; REGISTER_COUNT];

    'vm_loop: while let Some(routine) = routines.last_mut() {
        let pc = &mut routine.pc;
        let closure = closures.get(&routine.closure_addr).expect("invalid routine's closure address");
        let command = if let Some(command) = closure.get(*pc) {
            command
        } else {
            // no more commands left in this routine
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

        match command.kind {
            CommandKind::Dismiss => {}

            CommandKind::HaltVM => {
                break 'vm_loop;
            }

            CommandKind::Save => {
                let dest = command_arg!(0, Value::Address, "`dest` is not an Address");
                let dest = *dest as usize;

                check_address!(dest, "`dest` is out of bounds");

                let value = if let Some(value) = command.args.get(1) {
                    match value {
                        Value::Number(value) => *value,

                        Value::Address(addr) => {
                            let addr = *addr as usize;
                            check_address!(addr, "value address is out of bounds");
                            if let Value::Number(value) = registers[addr] {
                                value
                            } else {
                                error!("specified register's value is not a Number");
                                proceed!();
                            }
                        }

                        _ => {
                            error!("invalid value type");
                            proceed!();
                        }
                    }
                } else {
                    error!("missing value");
                    proceed!();
                };
                
                registers[dest] = Value::Number(value);
            }

            CommandKind::Add |
            CommandKind::Subtract |
            CommandKind::Multiply |
            CommandKind::Divide |
            CommandKind::IntDivide |
            CommandKind::Modulo |
            CommandKind::Power => {
                let src = command_arg!(0, Value::Address, "src is not an Address");
                let src = *src as usize;

                check_address!(src, "`src` is out of bounds");

                /* let right = if let Value::Number(value) = registers[src] {
                    value
                } else {
                    error!("src's value is not a Number");
                    proceed!();
                }; */

                let value = if let Some(value) = command.args.get(1) {
                    match value {
                        Value::Number(value) => *value,

                        Value::Address(addr) => {
                            let addr = *addr as usize;
                            check_address!(addr, "value address is out of bounds");
                            if let Value::Number(value) = registers[addr] {
                                value
                            } else {
                                error!("specified register's value is not a Number");
                                proceed!();
                            }
                        }

                        _ => {
                            error!("invalid value type");
                            proceed!();
                        }
                    }
                } else {
                    error!("missing value");
                    proceed!();
                };

                let dest = command_arg!(0, Value::Address, "`dest` is not an Address");
                let dest = *dest as usize;

                check_address!(dest, "`dest` is out of bounds");
                
                let (dest, left) = if let Value::Number(value) = registers[dest] {
                    (&mut registers[dest], value)
                } else {
                    error!("`dest`'s value is not a Number");
                    proceed!();
                };

                *dest = match command.kind {
                    CommandKind::Add => Value::Number(left + value),
                    CommandKind::Subtract => Value::Number(left - value),
                    CommandKind::Multiply => Value::Number(left * value),
                    CommandKind::Divide => Value::Number(left / value),
                    CommandKind::IntDivide => Value::Number((left / value).floor()),
                    CommandKind::Modulo => Value::Number(left % value),
                    CommandKind::Power => Value::Number(left.powf(value)),
                    CommandKind::Equal => Value::Boolean(left == value),
                    CommandKind::NotEqual => Value::Boolean(left != value),
                    CommandKind::GreaterThan => Value::Boolean(left > value),
                    CommandKind::GreaterThanOrEqual => Value::Boolean(left >= value),
                    CommandKind::LessThan => Value::Boolean(left < value),
                    CommandKind::LessThanOrEqual => Value::Boolean(left <= value),
                    _ => unreachable!()
                };
            }

            CommandKind::Equal |
            CommandKind::NotEqual |
            CommandKind::GreaterThan |
            CommandKind::GreaterThanOrEqual |
            CommandKind::LessThan |
            CommandKind::LessThanOrEqual => {
                let left = if let Some(value) = command.args.get(1) {
                    match value {
                        Value::Number(value) => *value,

                        Value::Address(addr) => {
                            let addr = *addr as usize;
                            check_address!(addr, "left value address is out of bounds");
                            if let Value::Number(value) = registers[addr] {
                                value
                            } else {
                                error!("specified register's value is not a Number");
                                proceed!();
                            }
                        }

                        _ => {
                            error!("invalid value type");
                            proceed!();
                        }
                    }
                } else {
                    error!("missing value");
                    proceed!();
                };

                let right = if let Some(value) = command.args.get(2) {
                    match value {
                        Value::Number(value) => *value,

                        Value::Address(addr) => {
                            let addr = *addr as usize;
                            check_address!(addr, "right's address is out of bounds");
                            if let Value::Number(value) = registers[addr] {
                                value
                            } else {
                                error!("specified register's value is not a Number");
                                proceed!();
                            }
                        }

                        _ => {
                            error!("invalid value type");
                            proceed!();
                        }
                    }
                } else {
                    error!("missing value");
                    proceed!();
                };

                let dest = command_arg!(0, Value::Address, "`dest` is not an Address");
                let dest = *dest as usize;

                if dest >= REGISTER_COUNT {
                    error!("`dest` is out of bounds");
                    proceed!();
                }

                let dest = &mut registers[dest];

                *dest = match command.kind {
                    CommandKind::Equal => Value::Boolean(left == right),
                    CommandKind::NotEqual => Value::Boolean(left != right),
                    CommandKind::GreaterThan => Value::Boolean(left > right),
                    CommandKind::GreaterThanOrEqual => Value::Boolean(left >= right),
                    CommandKind::LessThan => Value::Boolean(left < right),
                    CommandKind::LessThanOrEqual => Value::Boolean(left <= right),
                    _ => unreachable!()
                };
            }

            CommandKind::LogicalAnd |
            CommandKind::LogicalOr |
            CommandKind::LogicalNot => {
                let src = command_arg!(1, Value::Address, "`src` is not an Address");
                let src = *src as usize;

                check_address!(src, "`src` is out of bounds");
                
                let right = fetch_value!(src, Value::Boolean, "`src`'s value is not a Boolean");

                let dest = command_arg!(0, Value::Address, "`dest` is not an Address");
                let dest = *dest as usize;

                check_address!(dest, "`dest` is out of bounds");
                
                let (dest, left) = fetch_value_and_ptr!(dest, Value::Boolean, "`dest`'s value is not a Boolean");

                *dest = match command.kind {
                    CommandKind::LogicalAnd => Value::Boolean(left && right),
                    CommandKind::LogicalOr => Value::Boolean(left || right),
                    CommandKind::LogicalNot => Value::Boolean(!right),
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
