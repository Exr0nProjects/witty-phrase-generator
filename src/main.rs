extern crate getopts;
use getopts::Options;
use std::env;

use witty_phrase_generator::Generator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("n", "num", "set number of phrases to generate, default is 1", "NUM");
    opts.optopt("s", "sep", "set the character used between words, default is -", "CHAR");
    opts.optflag("2", "", "generate two word phrases (adjective-noun) instead of (intensifier-adjective-noun)");
    opts.optflag("4", "", "generate four word phrases (intensifier-adjective-noun-noun) instead"); // TODO: mutually exclusive group
    opts.optflag("a", "alliterate", "force words in a phrase to start with the same letter");
    opts.optflag("h", "help", "print this help menu then exit");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f.to_string())
    };

    if matches.opt_present("h") {
        println!("{}", opts.usage("Usage: witty-phrase-generator [options]"));
        return;
    }

    let words = if matches.opt_present("4") { 4 } else if matches.opt_present("2") { 2 } else { 3 };
    let num = matches.opt_get_default("n", 1)
                     .expect("Could not parse line count!");
    let sep = matches.opt_get_default("s", "-".to_string())
                     .expect("Could not parse separator!");

    let wp_gen = Generator::new();

    if let Some(phrases) = wp_gen.generic(3, 30, Some(20), Some(20), Some('a')) {
        for phrase in phrases {
            println!("{}", phrase.join(&sep));
        }
    }
    
    return ();

    // assert len > 0
    for _ in 0..num {
        let phrase = 'reroll: loop {
            // TODO: subtract total seperator length before passing
            let got = wp_gen.with_words(words as usize).expect("Empty word list!");
            if matches.opt_present("a") {
                for i in 1..words {
                    if got[i].chars().nth(0) != got[0].chars().nth(0) {
                        continue 'reroll;
                    }
                }
            }
            break got; 
        };
        println!("{}", phrase.join(&sep));
    };
}
