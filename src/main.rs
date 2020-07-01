use std::env;
use std::fs::File;
use std::io::Read;

pub mod encode;
mod node;

fn main() {
    // Get the commandline arguments
    let args: Vec<String> = env::args().collect();

    let option: String = args[1].parse().unwrap();
    let infile: String = args[2].parse().unwrap();
    let outfile: String = args[3].parse().unwrap();

    // Figure out which operation to preform
    match &option[..] {
        "-c" => compress(infile, outfile),
        "-d" => println!("decompressing"),
        "-v" => println!("encoding"),
        "-w" => println!("decoding"),
        _ => panic!("Usage: {} [-c|-d|-v|-w] infile outfile"),
    }
}

fn compress(infile: String, _outfile: String) {
    let mut f: File = File::open(infile).expect("Unable to open the infile");

    let mut bytes_array: Vec<u8> = Vec::new();

    // Get the bytes object of the infile
    f.read_to_end(&mut bytes_array)
        .expect("Unable to read the infile");

    encode::encode(bytes_array);
}
