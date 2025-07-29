use crate::Result;
use std::io::{self, Write};

pub fn prompt(message: &str) -> Result<String> {
    print!("{message}");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

pub fn select_from_list<T: std::fmt::Display>(
    message: &str,
    items: &[T],
    descriptions: Option<&[&str]>,
) -> Result<usize> {
    println!("{message}");

    for (i, item) in items.iter().enumerate() {
        if let Some(descs) = descriptions {
            if i < descs.len() {
                println!("  {}. {} - {}", i + 1, item, descs[i]);
            } else {
                println!("  {}. {}", i + 1, item);
            }
        } else {
            println!("  {}. {}", i + 1, item);
        }
    }

    loop {
        let input = prompt("Select an option: ")?;

        match input.parse::<usize>() {
            Ok(n) if n > 0 && n <= items.len() => return Ok(n - 1),
            _ => println!("Please enter a valid number between 1 and {}", items.len()),
        }
    }
}
