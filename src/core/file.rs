use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::core::advent_error::AdventError;
use std::result::Result;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines_to_vec<P: AsRef<Path>>(filename: P) -> Result<Vec<String>, AdventError>
{
    let br = read_lines(filename)?;

    let mut lines = Vec::<String>::new();

    for i in br {
        if let Ok(v) = i {
            lines.push(v);
        } else {
            return Result::Err(AdventError::OtherError("Unable to read a line from the file".to_string()));
        }
    }

    Ok(lines)
}