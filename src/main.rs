#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod ir;
mod vm;

fn main() {
    // импортируем ir, так как (почти) весь этот код потом будет в этом модуле
    use ir::*;

    let mut closures: Vec<Vec<Instruction>> = Vec::new();

    let main = vec![
    ];
            
    closures.push(main); /* 0 */

    let mut succeed = true;

    // closures validation
    for (cc, closure) in closures.iter().enumerate() {
        for (pc, command) in closure.iter().enumerate() {
            macro_rules! error {
                ($fmt:expr) => {
                    println!("IR: c{cc}:p{pc}:{}: {}", command.get_type_str(), $fmt);
                    succeed = false;
                };
            }

            macro_rules! check_register_ptr {
                ($addr:expr, $err_msg:expr) => {
                    if $addr >= vm::REGISTER_COUNT {
                        error!($err_msg);
                    }
                };
            }

            macro_rules! check_closure_ptr {
                ($addr:expr, $err_msg:expr) => {
                    if $addr >= closures.len() {
                        error!($err_msg);
                    }
                };
            }

            /* match command {
                Instruction::Store { dest, .. } => {
                    let dest = *dest as usize;
                    check_register_ptr!(dest, "`dest` register is out of bounds");
                }

                Instruction::Add { dest, src }
                | Instruction::Subtract { dest, src }
                | Instruction::Multiply { dest, src }
                | Instruction::Divide { dest, src }
                | Instruction::IntDivide { dest, src }
                | Instruction::Modulo { dest, src }
                | Instruction::Power { dest, src } => {
                    let src = *src as usize;
                    check_register_ptr!(src, "`src` register is out of bounds");

                    let dest = *dest as usize;
                    check_register_ptr!(dest, "`dest` register is out of bounds");
                }

                Instruction::Equal { dest, left, right }
                | Instruction::NotEqual { dest, left, right }
                | Instruction::GreaterThan { dest, left, right }
                | Instruction::GreaterThanOrEqual { dest, left, right }
                | Instruction::LessThan { dest, left, right }
                | Instruction::LessThanOrEqual { dest, left, right } => {
                    let left = *left;
                    check_register_ptr!(left, "`left` register is out of bounds");

                    let right = *right;
                    check_register_ptr!(right, "`right` register is out of bounds");

                    let dest = *dest;
                    check_register_ptr!(dest, "`dest` register is out of bounds");
                }

                Instruction::LogicalAnd { dest, src }
                | Instruction::LogicalOr { dest, src } => {
                    let src = *src;
                    check_register_ptr!(src, "`src` register is out of bounds");

                    let dest = *dest;
                    check_register_ptr!(dest, "`dest` register is out of bounds");
                }

                Instruction::LogicalNot { dest, src } => {
                    let src = *src;
                    check_register_ptr!(src, "`src` register is out of bounds");

                    let dest = *dest;
                    check_register_ptr!(dest, "`dest` register is out of bounds");
                }

                Instruction::Compare { register } => {
                    let register = *register;
                    check_register_ptr!(register, "`register` register is out of bounds");
                }

                Instruction::Jump { dest } => {
                    let dest = *dest;
                    check_closure_ptr!(dest, "`dest` is out of bounds");
                }

                Instruction::Call { closure, register_head } => {
                    let closure = *closure;
                    check_closure_ptr!(closure, "`closure` is out of bounds");

                    let register_head = *register_head;
                    check_register_ptr!(register_head, "`register_head` register is out of bounds");
                }

                _ => {}
            } */
        }
    }

    println!("compilation {}", if succeed { "succeed" } else { "failed" });

    if !succeed { return }
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
