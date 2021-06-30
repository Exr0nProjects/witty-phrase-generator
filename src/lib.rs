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

    /// Generate a phrase vector if possible
    ///
    /// returns None if the conditions could not be satisfied
    pub fn generic(&mut self,
                   words: usize,
                   count: usize,
                   len_min: Option<usize>,
                   len_max: Option<usize>,
                   start_char: Option<char>
                ) -> Option<Vec<Vec<&'static str>>> {

        if len_max < len_min        { return None }
        if words > 4 || words == 0  { return None }

        //let mut path_intensifiers = if start_char.is_some() {
        //    self.path_intensifiers.iter().filter(
        //        |x| x.chars().nth(0) == start_char
        //    ).collect::<Vec<&&'static str>>()
        //} else { self.path_intensifiers.iter().map(|x| x).collect() };
        //
        //let mut path_adjectives   = if start_char.is_some() {
        //    self.path_adjectives  .iter().filter(
        //        |x| x.chars().nth(0) == start_char
        //    ).collect::<Vec<&&'static str>>()
        //} else { self.path_adjectives  .iter().map(|x| x).collect() };
        //
        //let mut path_nouns        = if start_char.is_some() {
        //    self.path_nouns       .iter().filter(
        //        |x| x.chars().nth(0) == start_char
        //    ).collect::<Vec<&&'static str>>()
        //} else { self.path_nouns       .iter().map(|x| x).collect() };

        let mut path_intensifiers = self.path_intensifiers.iter().map(|x| x).collect::<Vec<&&'static str>>();
        let mut path_adjectives   = self.path_adjectives  .iter().map(|x| x).collect::<Vec<&&'static str>>();
        let mut path_nouns        = self.path_nouns       .iter().map(|x| x).collect::<Vec<&&'static str>>();

        if let Some(c) = start_char {
            path_intensifiers.retain(|s| s.chars().nth(0).expect("empty word found!") == c);
            path_adjectives  .retain(|s| s.chars().nth(0).expect("empty word found!") == c);
            path_nouns       .retain(|s| s.chars().nth(0).expect("empty word found!") == c);
        }

        if let Some(m) = len_max {
            path_intensifiers.retain(|s| s.len() < m);
            path_adjectives  .retain(|s| s.len() < m);
            path_nouns       .retain(|s| s.len() < m);
        }

        let mut ret = vec![vec![""; words]; count];




        Some(ret)
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

