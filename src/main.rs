
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::Read;
use std::env;

mod bf;

fn main() {
    let args: Vec<_> = env::args().collect();

    let content = read_file(&args[1]);

    let code = bf::BrainfuckParser::parse(&content);
    let mut interpreter = bf::BrainfuckInterpreter::new();
    interpreter.run(code);
}

fn read_file(filename: &str) -> String
{
    // Create a path to the desired file
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => s
    }
}
