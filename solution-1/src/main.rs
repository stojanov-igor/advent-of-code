use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}


fn main() -> Result<(), Error> {
    let vec = read(File::open("input1")?)?;

    let mut counter = 0;

    for (i, x) in vec.iter().enumerate() {
        if x < x {
            counter += 1;
        }
        println!("In position {} we have value {} {}", i, x, counter);
    }


    Ok(())
}
