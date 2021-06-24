extern crate getopts;
use getopts::Options;
use std::{ env, borrow::Cow };
use rand::prelude::{ ThreadRng, thread_rng, IteratorRandom };

static PATH_INTENSIFIERS: Cow<str> = String::from_utf8_lossy(include_bytes!("intensifiers.txt"));
static PATH_ADJECTIVES  : Cow<str> = String::from_utf8_lossy(include_bytes!("adjectives.txt")  );
static PATH_NOUNS       : Cow<str> = String::from_utf8_lossy(include_bytes!("nouns.txt")       );

fn gen(rng: &mut ThreadRng, num: usize) -> Vec<&'static str> {
    let mut ret = Vec::with_capacity(num);
    let mut n = 0;

    if num > 2 { ret[n] = PATH_INTENSIFIERS.lines().choose(&mut rng).unwrap(); n += 1; }
    if num > 1 { ret[n] = PATH_ADJECTIVES  .lines().choose(&mut rng).unwrap(); n += 1; }
    if num > 0 { ret[n] = PATH_NOUNS       .lines().choose(&mut rng).unwrap(); n += 1; }

    ret
}

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

    let mut rng: ThreadRng = thread_rng();
    let     num = matches.opt_get_default("n", 1)
                         .expect("Could not parse line count!");
    let     sep = matches.opt_get_default("s", "-".to_string())
                         .expect("Could not parse separator!");

    for _ in 0..num {
        println!("{}", gen(&mut rng, num as usize).join(&sep));
    };
}
