use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}


fn main() -> Result<(), Error> {
    let vec = read(File::open("input2")?)?;
    let mut stringVec = vec.clone(); 

    let mut currentCount = 0;

    // Add counting logic
    // String in vector and addition in another

    for x in vec.iter() {
        x.split_whitespace();
        println!("x={}, currentCount={}", x, currentCount);
    }


    Ok(())
}
