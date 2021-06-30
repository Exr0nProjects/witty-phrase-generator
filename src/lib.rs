use rand::prelude::{ ThreadRng, thread_rng, IteratorRandom };

pub struct Generator {
    rng: ThreadRng,
    path_intensifiers: Vec<&'static str>,
    path_adjectives: Vec<&'static str>,
    path_nouns: Vec<&'static str>,
}

impl Generator {
    pub fn new() -> Generator {
        let path_intensifiers = include_str!("intensifiers.txt");
        let path_adjectives   = include_str!("adjectives.txt")  ;
        let path_nouns        = include_str!("nouns.txt")       ;

        let path_intensifiers = path_intensifiers.lines().collect::<Vec<&'static str>>();
        let path_adjectives   = path_adjectives  .lines().collect::<Vec<&'static str>>();
        let path_nouns        = path_nouns       .lines().collect::<Vec<&'static str>>();

        Generator { 
            rng: thread_rng(),
            path_intensifiers,
            path_adjectives  ,
            path_nouns       ,
        }
    }

    /// Generate a witty phrase with either 1, 2, or 3 words
    ///
    /// returns None when no phrase could be generated (eg. if one of the wordlists is empty)
    pub fn with_words(&mut self, words: usize) -> Option<Vec<&'static str>> {
        let mut ret = vec![""; words];
        let mut n = 0;

        if words > 3 { ret[3] = self.path_nouns       .iter().choose(&mut self.rng)?; }

        if words > 2 { ret[n] = self.path_intensifiers.iter().choose(&mut self.rng)?; n += 1; }
        if words > 1 { ret[n] = self.path_adjectives  .iter().choose(&mut self.rng)?; n += 1; }
        if words > 0 { ret[n] = self.path_nouns       .iter().choose(&mut self.rng)?; }

        Some(ret)
    }
}

