use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("second arg: {}", args[1]);
    let text_file = &args[1];

    if let Ok(lines) = read_lines(text_file) {
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    };
    Ok(io::BufReader::new(file).lines())
}

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let text_file = &args[1];
//     let file = File::open(text_file);
//     let file = match file {
//         Ok(file) => file,
//         Err(error) => {
//             match error.kind() {
//                 std::io::ErrorKind::NotFound => {
//                     panic!("File not found: {}", error)
//                 }
//                 _ => {
//                     panic!("Error opening file: {}", error)
//                 }
//             }
//         }
//     };

//     let reader = BufReader::new(file);
//     for line in reader.lines() {
//         match line {
//             Ok(line) => println!("{}", line),
//             Err(error) => panic!("Error reading line: {}", error)
//         }
//     }
// }
