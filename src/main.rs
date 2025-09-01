mod cyclotomic;
mod cassels;

use cassels::loop_over_roots;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file_tables = File::create("tables.txt")?;
    let file_output = File::create("output.txt")?;
    let inputs = [(1, 1)];
    for (n0, len) in inputs {
        loop_over_roots(n0, len, &file_tables, &file_output);
    }

    let lines = BufReader::new(&file_output).lines();
    for line in lines {
        println!("{}", "This is a line");
    }

    println!("All cases checked!");
    Ok(())
}
