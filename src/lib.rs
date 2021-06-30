use rand::prelude::{ ThreadRng, thread_rng, IteratorRandom };

pub struct Generator {
    rng: ThreadRng,
    //path_intensifiers: &'static str,
    //path_adjectives: &'static str,
    //path_nouns: &'static str,
    //path_intensifiers: Lines,
    //path_adjectives: Lines,
    //path_nouns: Lines,
    //path_intensifiers: Vec<String>,
    //path_adjectives: Vec<String>,
    //path_nouns: Vec<String>,
    path_intensifiers: Vec<&'static str>,
    path_adjectives: Vec<&'static str>,
    path_nouns: Vec<&'static str>,
}

impl Generator {
    pub fn new() -> Generator {
        let path_intensifiers = include_str!("intensifiers.txt");
        let path_adjectives   = include_str!("adjectives.txt")  ;
        let path_nouns        = include_str!("nouns.txt")       ;

        //let path_intensifiers = path_intensifiers.lines().map(|x| x.to_string()).collect::<Vec<String>>();
        //let path_adjectives   = path_adjectives  .lines().map(|x| x.to_string()).collect::<Vec<String>>();
        //let path_nouns        = path_nouns       .lines().map(|x| x.to_string()).collect::<Vec<String>>();

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
    pub fn generic(&mut self, num: usize) -> Vec<&'static str> {
        let mut ret = vec![""; num];
        let mut n = 0;

        if num > 2 { ret[n] = self.path_intensifiers.iter().choose(&mut self.rng).unwrap(); n += 1; }
        if num > 1 { ret[n] = self.path_adjectives  .iter().choose(&mut self.rng).unwrap(); n += 1; }
        if num > 0 { ret[n] = self.path_nouns       .iter().choose(&mut self.rng).unwrap(); }

        ret
    }
}

