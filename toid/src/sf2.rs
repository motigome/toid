use std::fs::File;
use std::io::Read;

use toid::data::riff;
use toid::data::sf2;

fn main() {
    let mut f = File::open("../florestan-subset.sf2").unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    let buffer = buffer.as_slice();

    let chank = riff::RiffChank::parse(buffer);
    println!("{}", chank);

    let sf2_data = sf2::SF2::parse(buffer);
    println!("{}", sf2_data);
}