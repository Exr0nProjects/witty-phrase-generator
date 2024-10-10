extern crate getopts;
use getopts::Options;
use std::env;

use witty_phrase_generator::WPGen;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("n", "num", "set number of phrases to generate, default is 1", "NUM");
    opts.optopt("s", "sep", "set the character used between words, default is -", "CHAR");
    opts.optopt("c", "char", "specify alliteration char (shared by all phrases)", "CHAR");
    opts.optopt("w", "max-word-len", "specify the maximum word length, default is uncapped", "NUM");
    //opts.optopt("l", "length", "length of generated phrase") // TODO
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
    
    let max_word_len = matches.opt_get_default("w", usize::MAX)
                     .expect("Could not parse max word length!");

    let wp_gen = WPGen::new();

    if let Some(phrases) = if matches.opt_present("a") && !matches.opt_present("c") {
                wp_gen.with_phrasewise_alliteration(words, num, None, None, Some(max_word_len)) 
            } else {
                wp_gen.generic(words, num, None, None, Some(max_word_len), matches.opt_str("c")
                          .map(|x| x.chars().nth(0)
                              .expect("Must specify allitteration char!"))
                          .filter(|c| *c >= 'a' && *c <= 'z'))
            } {
        for p in phrases {
            println!("{}", p.join(&sep))
        }
    }
}
