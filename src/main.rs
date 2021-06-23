use std::{ env, fs, io };

use std::io::BufRead;
use std::fs::File;
use std::path::Path;

use rand::prelude::{ thread_rng, IteratorRandom };

//fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
//where P: AsRef<Path>, {
//    // https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
//    let file = File::open(filename)?;
//    Ok(io::BufReader::new(file).lines())
//}

fn main() {
    //let path_intensifiers = read_lines("intensifiers.txt").expect("Error loading intensifiers!");
    //let path_adjectives   = read_lines("adjectives.txt")  .expect("Error loading adjectives!");
    //let path_nouns        = read_lines("nouns.txt")       .expect("Error loading nouns!");

    let path_intensifiers = String::from_utf8_lossy(include_bytes!("intensifiers.txt"));
    let path_adjectives   = String::from_utf8_lossy(include_bytes!("adjectives.txt")  );
    let path_nouns        = String::from_utf8_lossy(include_bytes!("nouns.txt")       );

    let mut rng = thread_rng();
    println!("random intensifier: {}", path_intensifiers.lines().choose(&mut rng).unwrap());
}
