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
    let mut forward_count= 0;
    let mut down_count = 0;
    let mut up_count = 0;
    let mut number_end = 0;

    for x in vec.iter() {
        //let y = x.split_whitespace();
        let char_vec: Vec<char> = x.chars().collect();
        let number_end: Vec<u32> = x.chars().rev().take(1).flat_map(|ch| ch.to_digit(10)).collect();

        let ch = char_vec[0];
        if ch == 'f' {
            forward_count += 1;
            // number_end += number_end;
        }
        if ch == 'd' {
            down_count += 1;
        }
        if ch == 'u' {
            up_count += 1;
        }

        println!("forward={:?}, down={:?}, up={:?}, Total_forward: {:?}",  forward_count, down_count, up_count, number_end);
    }


    Ok(())
}
