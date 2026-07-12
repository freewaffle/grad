#![forbid(unsafe_code)]
#![deny(clippy::all)]

#[allow(dead_code)]
mod vm {
    pub const REGISTER_COUNT: usize = 128;
    pub const MAX_ROUTINE_COUNT: usize = 256;

    pub mod ptr {
        pub type Register = u8;
        pub type Instruction = u16;
        pub type Closure = u8;
    }

    #[derive(Clone, Copy, Debug)]
    pub enum Value {
        Nil,
        Number(f32),
        Boolean(bool),
    }

    pub enum RoutineArgument {
        Reference(ptr::Register),
        Literal(Value)
    }

    #[repr(u8)]
    pub enum Command {
        NoOp,
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
        Call {
            closure: ptr::Closure,
            register_head: ptr::Register,
        }
    }

    impl Command {
        pub fn get_type_str(&self) -> &'static str {
            match self {
                Command::NoOp => "NoOp",
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
                Command::Break => "Break",
                Command::Call { .. } => "Call"
            }
        }
    }

    pub struct Routine {
        pub closure_addr: ptr::Closure,
        pub register_head: ptr::Register,
        pub pc: usize
    }

    impl Routine {
        #[inline]
        pub fn new(closure_addr: ptr::Closure, register_head: ptr::Register) -> Self {
            Self {
                closure_addr,
                register_head,
                pc: 0
            }
        }
    }
}

fn main() {
    use vm::*;

    let mut closures: Vec<Vec<Command>> = Vec::new();
    
    let extra_closure = vec![
        Command::Store { dest: 2, value: Value::Number(6.) },
        Command::Multiply { dest: 2, src: 2 },
        Command::Add { dest: 2, src: 0 },
    ];

    let main = vec![
        Command::Store { dest: 0, value: Value::Number(4.) },
        Command::Store { dest: 1, value: Value::Number(4.) },
        Command::Multiply { dest: 1, src: 0 },
        Command::Add { dest: 1, src: 0 },
        Command::Call { closure: 1, register_head: 0 },
    ];
            
    closures.push(main); /* 0 */
    closures.push(extra_closure); /* 1 */

    let mut routine_stack: Vec<Routine> = vec![
        Routine::new(0, 0)
    ];

    let mut registers = [Value::Nil; REGISTER_COUNT];

    let mut code_ok = true;

    for (cc, closure) in closures.iter().enumerate() {
        for (pc, command) in closure.iter().enumerate() {
            macro_rules! error {
                ($fmt:expr) => {
                    eprintln!("GVM: c{cc}:p{pc}:{}: {}", command.get_type_str(), $fmt);
                    code_ok = false;
                };
            }

            macro_rules! check_register_ptr {
                ($addr:expr, $err_msg:expr) => {
                    if $addr >= REGISTER_COUNT {
                        error!($err_msg);
                    }
                };
            }

            match command {
                Command::Store { dest, .. } => {
                    let dest = *dest as usize;
                    if dest >= REGISTER_COUNT {
                        error!("`dest` register is out of bounds");
                    }
                }

                Command::Add { dest, src }
                | Command::Subtract { dest, src }
                | Command::Multiply { dest, src }
                | Command::Divide { dest, src }
                | Command::IntDivide { dest, src }
                | Command::Modulo { dest, src }
                | Command::Power { dest, src } => {
                    let src = *src as usize;
                    check_register_ptr!(src, "`src` register is out of bounds");

                    let dest = *dest as usize;
                    check_register_ptr!(dest, "`dest` register is out of bounds");
                }

                Command::Equal { dest, left, right }
                | Command::NotEqual { dest, left, right }
                | Command::GreaterThan { dest, left, right }
                | Command::GreaterThanOrEqual { dest, left, right }
                | Command::LessThan { dest, left, right }
                | Command::LessThanOrEqual { dest, left, right } => {
                    let left = *left as usize;
                    check_register_ptr!(left, "`left` register is out of bounds");

                    let right = *right as usize;
                    check_register_ptr!(right, "`right` register is out of bounds");

                    let dest = *dest as usize;
                    check_register_ptr!(dest, "`dest` register is out of bounds");
                }

                Command::LogicalAnd { dest, src }
                | Command::LogicalOr { dest, src } => {
                    let src = *src as usize;
                    check_register_ptr!(src, "`src` register is out of bounds");

                    let dest = *dest as usize;
                    check_register_ptr!(dest, "`dest` register is out of bounds");
                }

                Command::LogicalNot { dest, src } => {
                    let src = *src as usize;
                    check_register_ptr!(src, "`src` register is out of bounds");

                    let dest = *dest as usize;
                    check_register_ptr!(dest, "`dest` register is out of bounds");
                }

                Command::Compare { register } => {
                    let register = *register as usize;
                    check_register_ptr!(register, "`register` register is out of bounds");
                }

                Command::Jump { dest } => {
                    let dest = *dest as usize;
                    if dest >= closure.len() {
                        error!("`dest` instruction is out of bounds");
                    }
                }

                Command::Call { closure, register_head } => {
                    let closure = *closure as usize;
                    if closure >= closures.len() {
                        error!("`closure` is out of bounds");
                    }

                    let register_head = *register_head as usize;
                    check_register_ptr!(register_head, "`register_head` register is out of bounds");
                }

                _ => {}
            }
        }
    }

    if !code_ok { return }

    'vm_loop: while !routine_stack.is_empty() {
        let routine_count = routine_stack.len();

        let routine = routine_stack.last_mut().unwrap();

        let pc = &mut routine.pc;
        let closure_addr = routine.closure_addr;
        let closure = &closures[closure_addr as usize];
        let command = if let Some(command) = closure.get(*pc) {
            command
        } else {
            // no more commands left in this routine
            routine_stack.pop();
            continue 'vm_loop;
        };

        let mut pushing_routine: Option<Routine> = None;

        macro_rules! next_command {
            () => {
                if let Some(new_routine) = pushing_routine {
                    routine_stack.push(new_routine);
                }

                continue 'vm_loop;
            };
        }

        macro_rules! error {
            ($fmt:expr) => {
                eprintln!("GVM: c{closure_addr}:p{pc}:{}: {}", command.get_type_str(), $fmt);
            };
        }

        macro_rules! proceed {
            () => {
                *pc += 1;
                next_command!();
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

        match command {
            Command::NoOp => {}

            Command::HaltVM => {
                break 'vm_loop;
            }

            Command::Store { dest, value } => {
                registers[*dest as usize] = *value;
            }

            Command::Add { dest, src }
            | Command::Subtract { dest, src }
            | Command::Multiply { dest, src }
            | Command::Divide { dest, src }
            | Command::IntDivide { dest, src }
            | Command::Modulo { dest, src }
            | Command::Power { dest, src } => {
                let src = *src as usize;
                let value: f32 = if let Value::Number(value) = registers[src] {
                    value
                } else {
                    error!("specified register's value is not a Number");
                    proceed!();
                };

                let dest = *dest as usize;
                let (dest, dest_value) = fetch_value_and_ptr!(dest, Value::Number, "`dest` register's value is not a Number");

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

            Command::Equal { dest, left, right }
            | Command::NotEqual { dest, left, right }
            | Command::GreaterThan { dest, left, right }
            | Command::GreaterThanOrEqual { dest, left, right }
            | Command::LessThan { dest, left, right }
            | Command::LessThanOrEqual { dest, left, right } => {
                let left = *left as usize;
                let left: f32 = if let Value::Number(value) = registers[left] {
                    value
                } else {
                    error!("`left` register's value is not a Number");
                    proceed!();
                };

                let right = *right as usize;
                let right: f32 = if let Value::Number(value) = registers[right] {
                    value
                } else {
                    error!("`right` register's value is not a Number");
                    proceed!();
                };

                let dest = *dest as usize;
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

            Command::LogicalAnd { dest, src }
            | Command::LogicalOr { dest, src } => {
                let src = *src as usize;
                let src = fetch_value!(src, Value::Boolean, "`src` register's value is not a Boolean");

                let dest = *dest as usize;
                let (dest, dest_value) = fetch_value_and_ptr!(dest, Value::Boolean, "`dest` register's value is not a Boolean");

                *dest = match command {
                    Command::LogicalAnd { .. } => Value::Boolean(dest_value && src),
                    Command::LogicalOr { .. } => Value::Boolean(dest_value || src),
                    _ => unreachable!()
                };
            }

            Command::LogicalNot { dest, src } => {
                let src = *src as usize;
                let src = fetch_value!(src, Value::Boolean, "`src` register's value is not a Boolean");

                let dest = *dest as usize;
                registers[dest] = Value::Boolean(!src);
            }

            /* compare, beginning, break */

            Command::Compare { register } => {
                let register = *register as usize;
                let value = fetch_value!(register, Value::Boolean, "`register` register's value is not a Boolean");

                if !value {
                    *pc += 1;
                }
            }

            Command::Jump { dest } => {
                let dest = *dest as usize;
                *pc = dest;
                next_command!();
            }

            Command::JumpToBeginning => {
                *pc = 0;
                next_command!();
            }

            Command::Break => {
                routine_stack.pop();
                next_command!();
            }

            Command::Call { closure, register_head } => {
                if routine_count >= MAX_ROUTINE_COUNT {
                    error!("routine stack overflow");
                    proceed!();
                }

                let new_routine = Routine::new(*closure, *register_head);
                // routine_stack.push(new_routine);
                pushing_routine = Some(new_routine);
            }
        }

        proceed!();
    }

    println!("register 1 = {:?}", registers[1]);
    println!("register 2 = {:?}", registers[2]);
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
