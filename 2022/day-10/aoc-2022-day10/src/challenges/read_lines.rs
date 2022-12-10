use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};
use std::path::Path;
use std::result::Result;

pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>, Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename);
    match file {
        Ok(file) => Ok(BufReader::new(file).lines()),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}
