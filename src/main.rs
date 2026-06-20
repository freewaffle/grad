#![forbid(unsafe_code)]
#![deny(clippy::all)]

#[allow(dead_code)]
mod vm {
    pub const REGISTER_COUNT: usize = 128;

    #[derive(Clone, Copy, Debug)]
    pub enum Value {
        Nil,
        Number(f32),
        Address(u8)
    }

    #[derive(Clone, Copy, Debug)]
    #[repr(u8)]
    pub enum CommandKind {
        Dismiss,
        HaltVM,

        SaveRegister,
        CopyRegister,

        Add,
        Subtract,
        Multiply,
        Divide,
    }
    
    pub struct Command {
        pub kind: CommandKind,
        pub args: Vec<Value>
    }
}

fn main() {
    use vm::*;

    let commands = [
        Command {
            kind: CommandKind::SaveRegister,
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

    let mut registers = [vm::Value::Nil; vm::REGISTER_COUNT];

    let mut pc: usize = 0;
    'vm_loop: loop {
        if pc >= commands.len() {
            break;
        }

        let command = &commands[pc];

        macro_rules! error {
            ($fmt:expr) => {
                eprintln!("GVM:{}:{:?}: {}", pc, command.kind, $fmt);
            };
        }

        macro_rules! proceed {
            () => {
                pc += 1;
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

            CommandKind::SaveRegister => {
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

            CommandKind::CopyRegister => {
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
            CommandKind::Divide => {
                let dest = command_arg!(0, vm::Value::Address, "dest is not an Address");
                let dest = *dest as usize;

                let value = command_arg!(1, vm::Value::Number, "value is not a Number");
                let value = *value;

                if dest >= vm::REGISTER_COUNT {
                    error!("dest is out of bounds");
                    proceed!();
                }
                
                let (dest, dest_value) = if let vm::Value::Number(value) = registers[dest] {
                    (&mut registers[dest], value)
                } else {
                    error!("dest's value is not a Number");
                    proceed!();
                };

                let new_value = match command.kind {
                    CommandKind::Add => dest_value + value,
                    CommandKind::Subtract => dest_value - value,
                    CommandKind::Multiply => dest_value * value,
                    CommandKind::Divide => dest_value / value,
                    _ => unreachable!()
                };

                *dest = vm::Value::Number(new_value);
            }
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
