use same_file::Handle;
use std::fs::File;
use std::io::{Write, BufReader, Error, ErrorKind};
use std::path::Path;


fn main() -> Result<(), Error> {
    let path = "line.txt";

    let mut output = File::create(path)?;
    write!(output, "Rust\nis\nFun")?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}
