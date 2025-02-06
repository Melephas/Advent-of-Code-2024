use std::fs::{read_to_string, File};
use std::iter;
use std::iter::zip;
use std::str::FromStr;
use anyhow::Result;
use thiserror::Error;
use crate::ParseError::{InvalidLine, MismatchedLengths};

const INPUT: &str = "day1-input";


#[derive(Error, Debug)]
enum ParseError {
  #[error("Invalid input on line {0}")]
  InvalidLine(usize),
  #[error("Mismatched list lengths ({0} vs {1})")]
  MismatchedLengths(usize, usize),
}

fn main() -> Result<()> {
  let (mut left, mut right) = read_input()?;
  
  left.sort();
  right.sort();
  
  if left.len() != right.len() {
    Err(MismatchedLengths(left.len(), right.len()))?
  }
  
  let zipped = left.iter().zip(right.iter());
  let mut sum = 0;
  for (l, r) in zipped {
    if l > r {
      sum += l - r;
    } else {
      sum += r - l;
    }
  }
  
  println!("{}", sum);
  
  Ok(())
}

fn read_input() -> Result<(Vec<u32>, Vec<u32>)> {
  let input = read_to_string(INPUT)?;
  let lines = input.lines().collect::<Vec<&str>>();
  
  let mut left = Vec::new();
  let mut right = Vec::new();
  
  for (idx, line) in lines.iter().enumerate() {
    let vals = line.split_ascii_whitespace().collect::<Vec<&str>>();
    if vals.len() != 2 {
      Err(InvalidLine(idx + 1))?;
    }
    left.push(u32::from_str(vals[0])?);
    right.push(u32::from_str(vals[1])?);
  }
  
  Ok((left, right))
}
