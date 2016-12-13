use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    println!("Answer: {}", solve("large.in", 8).unwrap());
}

fn solve(file_name: &str, line_len: i32) -> Result<&str, io::Error> {
    let file = File::open(file_name)?;
    let input = BufReader::new(file);
    let mut freq_lists = Vec::new();
    for _ in 0..line_len {
        freq_lists.push(HashMap::<&str, i32>::new());
    }
    println!("{:?}", freq_lists.len());
    
    for line in input.lines() {
        println!("{:?}", line.unwrap());
    }
    Ok("ok")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn small() {
        assert_eq!(solve("small.in", 6).unwrap(), "easter");
    }
}