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
    let mut vec2 = vec.clone();

    // remove first element of sec vector
    vec2.remove(0);
    let mut counter = 0;

    for (x, y) in vec.iter().zip(vec2.iter()) {
        if y > x {
            counter += 1;
        }
        println!("x={}, y={}, bigger={}", x, y, counter);
    }


    Ok(())
}
