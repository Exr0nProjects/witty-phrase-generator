extern crate getopts;
use getopts::Options;
use std::env;

use witty_phrase_generator::Generator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("n", "num", "set number of phrases to generate", "NUM");
    opts.optopt("s", "sep", "set the character used between words, default is -", "CHAR");
    opts.optflag("2", "", "generate a two word phrase (adjective-noun)");
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

    let len = if matches.opt_present("2") { 2 } else { 3 };
    let num = matches.opt_get_default("n", 1)
                     .expect("Could not parse line count!");
    let sep = matches.opt_get_default("s", "-".to_string())
                     .expect("Could not parse separator!");

    let mut rng: ThreadRng = thread_rng();
    // assert len > 0
    for _ in 0..num {
        let phrase = 'reroll: loop {
            let got = gen(&mut rng, len as usize, &path_intensifiers, &path_adjectives, &path_nouns);
            if matches.opt_present("a") {
                for i in 1..len {
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
