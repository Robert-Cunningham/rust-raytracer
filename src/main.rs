use std::fs::File;
use std::io::prelude::*;
use std::ops;
use std::fmt;

mod vec3;

fn main() {
    let x: &str = "Hello World!";
    println!("{}", x);
    save_image(20, 10);
    println!("{}", vec3::vec3{x: 1.0, y: 2.0, z: 4.0} * vec3::vec3{x: 1.0, y: 4.0, z: 3.0})
}

fn save_image(x : i32, y : i32) -> std::io::Result<()> {
    let mut file = File::create("foo.ppm")?;
    file.write_all(b"P3\n")?;
    file.write_all(format!("{} {}\n", x, y).as_bytes())?;
    file.write_all(b"255\n")?;

    for j in 0..y {
        for i in 0..x {
            let r = 255 * i / x as i32;
            let g = 255 * j / y as i32;
            let b = 255 * 0.5 as i32;
            file.write_all(format!("{} {} {} \n", r, g, b).as_bytes())?;
        }
        //file.write_all(b"\n");
    }
    Ok(())
}