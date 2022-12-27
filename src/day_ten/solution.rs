use super::instruction::Instruction;
use super::cpu::Cpu;
use super::pixel::PixelState;

use std::collections::HashSet;

fn convert_input_to_commands(data: Vec<String>) -> Vec<Instruction> {
    data.into_iter().map(Instruction::parse).collect()
}

fn get_required_cycles() -> HashSet<i32> {
    vec![20, 60, 100, 140, 180, 220].into_iter().collect()
}

pub fn calculate_part_one(data: Vec<String>) -> i32 {
    let commands = convert_input_to_commands(data);
    let initial_cpu = Cpu { register_value: 1};
    
    let mut current_state = initial_cpu.clone();
    let mut seen_states = vec![initial_cpu];

    for command in commands.iter() {
        let next_states = current_state.apply_instruction(command);
        for state in next_states {
            seen_states.push(state.clone());
            current_state = state;
        }
    }

    let required_cycles = get_required_cycles();

    let cycle_sum: i32 =
        // We start from 1 because we care about the values *during* the cycle, not after the cycle has finished.
        // Therefore, the value during the nth cycle will be the value at the end of the (n-1)th cycle.
        (1 ..)
            .into_iter()
            .zip(seen_states.into_iter())
            .filter(|(i, _)| {
                required_cycles.contains(i)
                //true
            })
            .map(|(i, cpu)| {
                //println!("{}th cycle has value of {}", i, cpu.register_value);
                i * cpu.register_value
            })
            .sum();

    cycle_sum
}

fn print_screen(screen: &Vec<Vec<PixelState>>) -> () {
    for row in screen.iter() {
        for pixel in row.iter() {
            let pixel_value =
                match pixel {
                    PixelState::Off => '.',
                    PixelState::On => '#'
                };
            print!("{pixel_value}");
        }
        println!("");
    }
}

pub fn calculate_part_two(data: Vec<String>) -> i32 {
    let commands = convert_input_to_commands(data);
    let initial_cpu = Cpu { register_value: 1};
    
    let mut current_state = initial_cpu.clone();
    let mut seen_states = vec![initial_cpu];

    for command in commands.iter() {
        let next_states = current_state.apply_instruction(command);
        for state in next_states {
            seen_states.push(state.clone());
            current_state = state;
        }
    }

    let mut screen: Vec<Vec<PixelState>> = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ];

    // We start from 1 because we care about the values *during* the cycle, not after the cycle has finished.
    // Therefore, the value during the nth cycle will be the value at the end of the (n-1)th cycle.
    (1 ..)
        .into_iter()
        .zip(seen_states.into_iter().take(240))
        .for_each(|(i, cpu)| {
            let index = (i - 1) % 40;
            let row = (i-1) / 40;
            let state =
                if index >= cpu.register_value - 1 && index <= cpu.register_value + 1 {
                    PixelState::On
                } else {
                    PixelState::Off
                };
            if row > 5 {
                return;
            }
            println!("{index} / {} = {row}", 40);
            screen[row as usize].push(state);
        });
    print_screen(&screen);
    0
}