use std::fs::File;
use std::io::prelude::*;


fn main() {
    let x: &str = "Hello World!";
    println!("{}", x);
    save_image();
}

fn save_image() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(b"This is a test")?;
    Ok(())
}