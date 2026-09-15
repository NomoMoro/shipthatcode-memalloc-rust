use std::io::{self, BufRead};
mod bump;
use bump::BumpAllocator;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
// TODO (why-allocator): implement per the lesson description.

enum AllocatorCommands {
    INIT(usize),
    ALLOC(usize),
    RESET,
    USED,
}

fn get_size_part(line: Vec<&str>) -> usize {
    return if let Some(size) = line.get(1) {
        size.parse::<usize>().unwrap_or(0)
    } else {
        0 as usize
    };
}

fn parse_command(line: &str) -> Result<AllocatorCommands, GenericError> {
    let line_parts: Vec<&str> = line.split_whitespace().collect();
    let command_part = if let Some(first) = line_parts.first() {
        *first
    } else {
        "none"
    };
    return match command_part {
        "INIT" => {
            let size_part: usize = get_size_part(line_parts);
            Ok(AllocatorCommands::INIT(size_part))
        }
        "ALLOC" => {
            let size_part: usize = get_size_part(line_parts);
            Ok(AllocatorCommands::ALLOC(size_part))
        }
        "RESET" => Ok(AllocatorCommands::RESET),
        "USED" => Ok(AllocatorCommands::USED),
        _ => Err("Unknown command".into()),
    };
}

fn main() {
    let mut bump_alloc: BumpAllocator = BumpAllocator::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() {
            continue;
        }
        let parse_result: AllocatorCommands = if let Ok(command) = parse_command(l.as_str()) {
            command
        } else {
            continue;
        };
        match parse_result {
            AllocatorCommands::INIT(size) => {
                bump_alloc = BumpAllocator::init(size);
                println!("OK")
            }
            AllocatorCommands::ALLOC(size) => {
                let output = bump_alloc
                    .alloc(size)
                    .map(|addr| addr.to_string())
                    .unwrap_or_else(|err| err.to_string());

                println!("{}", output);
            }
            AllocatorCommands::RESET => {
                bump_alloc.reset();
                println!("OK");
            }
            AllocatorCommands::USED => {
                println!("{}", bump_alloc.used());
            }
        }
    }
}
