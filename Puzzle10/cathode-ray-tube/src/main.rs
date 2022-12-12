use cathode_ray_tube::cpu::CPU;
use cathode_ray_tube::instructions::Instruction;
use clap::Parser;
use std::fs;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, short)]
    input: String,
}
fn main() {
    const MAX_CYCLES: u32 = 240;
    const FIRST_STEP: u32 = 20;
    const STEP: u32 = 40;
    let args = Args::parse();
    let instructions_str = fs::read_to_string(args.input).unwrap();
    let instructions = Instruction::load_instructions(instructions_str, 2);
    let mut cpu = CPU::new(instructions, STEP);
    let mut signal_strength = 0;
    for i in 1..(MAX_CYCLES + 1) {
        cpu.consume_next_operation();
        print!("{}", cpu.current_char());
        if i % STEP == 0 {
            println!("");
        }
        if i == FIRST_STEP || i % STEP == FIRST_STEP {
            /*println!(
                "Register value at cycle {}: {}, Strength: {}",
                cpu.cycles(),
                cpu.register_value(),
                (cpu.cycles() as i32) * cpu.register_value()
            );*/
            signal_strength += cpu.register_value() * (cpu.cycles() as i32);
        }
    }
    println!("Signal strenght at the end: {}", signal_strength);
}
