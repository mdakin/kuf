use std::env;
use std::fs::File;
// prelude? I think this is poor naming. But maybe it is a convention.
// std::io::prelude is a module basically packages a bunch of io related 
// traits, namey these:
//
// std::io::Read;
// std::io::Write;
// std::io::::BufRead;
// std::io::Seek;
// 
use std::io::prelude::*;

fn dump_file_str(filename: &String) {
    let mut f = File::open(filename).expect("file not found");
    // String is mutable (somewhat similar to a StringBuilder in Java)
    let mut contents = String::new();
    // read_to_string is from the trait std::io::Read
    // Why do we need to pass the reference as mutable as well?
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    println!("With text:\n{}", contents);
}

fn dump_file_binary(filename: &String) {
    let mut f = File::open(filename).expect("file not found");
    // vec! is a macro to initialize vectors. vec![0; 10] creates a vector of
    // size 10 all initialized to 0. The type of buffer is Vec<i32> or Vec<u8>?
    // Not sure which types are assigned from int literals. Maybe it decides it
    // is Vec<u8> because later we use it in the read_to_end method which has the
    // signature:
    // fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize>
    let mut buffer = vec![0];
    // read the whole file into the buffer 
    f.read_to_end(&mut buffer).expect("Could not read fie.");
    // :x? or :X?  seems to be a formatter "debug with hexadecimal integers"
    println!("{:X?}", buffer);
    // TBD: Pretty print hex dump.
}

fn main() {
    // Get args. First argument is always program name so we skip it.
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        println!("Please provide a file name as an argument.");
        ::std::process::exit(0);
    }
    let file_name = &args[0];
    println!("Dumping file: {}", file_name);
    dump_file_str(file_name);
    dump_file_binary(file_name);
}
