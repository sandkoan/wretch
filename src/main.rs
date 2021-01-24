use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::io::{Read, BufReader};
use std::fs::File;


fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, E> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}


fn main() -> Result<(), E> {
    let path = "file.txt";

    let mut output = File::create(path)?;
    write!(output, "Hello World")?;

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;

    println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()));

    Ok(())
}
