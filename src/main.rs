use crate::interpreter::Chip8VM;

mod interpreter;

fn main() {
	println!("Hello Chip-8 Rusty VM!");
	let mut vm = Chip8VM::new();
	chip8_base::run(vm);
}