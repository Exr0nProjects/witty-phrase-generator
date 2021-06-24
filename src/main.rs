extern crate getopts;
use getopts::Options;
use std::env;
use rand::prelude::{ thread_rng, IteratorRandom };

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("n", "num", "set number of phrases to generate", "NUM");
    opts.optopt("s", "sep", "set the character used between words, default is -", "CHAR");
    opts.optflag("2", "", "generate a two word phrase (adjective-noun)");
    opts.optflag("h", "help", "print this help menu then exit");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f.to_string())
    };

    if matches.opt_present("h") {
        println!("{}", opts.usage("Usage: witty-phrase-generator [options]"));
        return;
    }

    let mut rng = thread_rng();
    let     sep = matches.opt_get_default("s", "-".to_string())
                         .expect("Could not parse separator!");

    if !matches.opt_present("2") {
        let path_intensifiers = String::from_utf8_lossy(include_bytes!("intensifiers.txt"));
        print!("{}{}", path_intensifiers.lines().choose(&mut rng).unwrap(), sep);
    };

    let path_adjectives   = String::from_utf8_lossy(include_bytes!("adjectives.txt")  );
    let path_nouns        = String::from_utf8_lossy(include_bytes!("nouns.txt")       );

    println!("{}{}{}{}{}{}{}{}{}{}{}",
             path_adjectives  .lines().choose(&mut rng).unwrap(),
             sep,
             path_nouns       .lines().choose(&mut rng).unwrap(),
             path_nouns       .lines().choose(&mut rng).unwrap(),
             path_nouns       .lines().choose(&mut rng).unwrap(),
             path_nouns       .lines().choose(&mut rng).unwrap(),
             path_nouns       .lines().choose(&mut rng).unwrap(),
             path_nouns       .lines().choose(&mut rng).unwrap(),
             path_nouns       .lines().choose(&mut rng).unwrap(),
             path_nouns       .lines().choose(&mut rng).unwrap(),
             path_nouns       .lines().choose(&mut rng).unwrap(),
        );
}
