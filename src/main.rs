use std::io::{self, BufRead};
type GenericError = Box<dyn std::error::Error + Send + Sync>;
// TODO (why-allocator): implement per the lesson description.

enum AllocatorCommands {
    INIT(usize),
    ALLOC(usize),
    RESET,
    USED,
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
            let size_part: usize = if let Some(size) = line_parts.get(1) {
                size.parse::<usize>()?
            } else {
                0
            };
            Ok(AllocatorCommands::INIT(size_part))
        }
        "ALLOC" => {
            todo!()
        }
        "RESET" => {
            todo!()
        }
        "USED" => {
            todo!()
        }
        _ => Err("Unknown command".into()),
    };
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() {
            continue;
        }
        println!("TODO");
    }
}
