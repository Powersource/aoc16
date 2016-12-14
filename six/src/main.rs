use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    println!("Answer: {}", solve("large.in", 8).unwrap());
}

fn solve(file_name: &str, line_len: i32) -> Result<String, io::Error> {
    let file = File::open(file_name)?;
    let input = BufReader::new(file);
    let mut freq_lists = Vec::new();
    for _ in 0..line_len {
        freq_lists.push(HashMap::<char, i32>::new());
    }
    println!("{:?}", freq_lists.len());

    for line in input.lines() {
        for (i, c) in line.unwrap().chars().enumerate() {
            *freq_lists[i].entry(c).or_insert(0) += 1;
        }
    }
    let mut winners = Vec::new();
    for hm in freq_lists.into_iter() {
        winners.push(hm.into_iter()
            .fold(('_', -1),
                  |prev, next| if prev.1 > next.1 { prev } else { next }));
    }
    Ok(format!("{}",
               winners.into_iter().map(|pair| pair.0).collect::<String>()))
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn small() {
        assert_eq!(solve("small.in", 6).unwrap(), "easter".to_string());
    }
}