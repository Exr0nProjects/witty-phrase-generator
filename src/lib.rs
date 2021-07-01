use rand::prelude::{ ThreadRng, thread_rng, IteratorRandom };
use rand::seq::SliceRandom;

use std::cell::RefCell;

pub struct Generator {
    rng: RefCell<ThreadRng>,
    words_intensifiers: Vec<&'static str>,
    words_adjectives: Vec<&'static str>,
    words_nouns: Vec<&'static str>,
}

impl Generator {
    pub fn new() -> Generator {
        let words_intensifiers = include_str!("intensifiers.txt");
        let words_adjectives   = include_str!("adjectives.txt")  ;
        let words_nouns        = include_str!("nouns.txt")       ;

        let words_intensifiers = words_intensifiers.lines().collect::<Vec<&'static str>>();
        let words_adjectives   = words_adjectives  .lines().collect::<Vec<&'static str>>();
        let words_nouns        = words_nouns       .lines().collect::<Vec<&'static str>>();

        Generator { 
            rng: RefCell::new(thread_rng()),
            words_intensifiers,
            words_adjectives  ,
            words_nouns       ,
        }
    }

    fn create_format(words: usize) -> Vec<usize> {
        // TODO: return Vec<ENUM{ intensifier, adjective, noun }> instead of usize
        // TODO: convert with_words fn to use this also

        let mut ret = vec![0; words+1];
        let mut n = 1;

        if words > 3 { ret[3] = 3 }
        if words > 2 { ret[n] = 1; n += 1; }
        if words > 1 { ret[n] = 2; n += 1; }
        if words > 0 { ret[n] = 3; }
         
        ret
    }

    fn generate_backtracking(&self,
                             len_min: usize,
                             len_max: usize,
                             dep: usize,
                             shift: &mut [usize; 4],    // which words of each position we've already used up (TODO: broken, since binary search window -> not contiguous usage, should technically be a hashset)
                             dict: &[Vec<&&'static str>; 4],
                             format: &Vec<usize>,
                            ) -> Option<Vec<&'static str>> {
        println!("dict {} format {} dep {}", dict.len(), format.len(), dep);
        let pool = &dict[format[dep]];
        
        let upper_bound = {
            let [mut l, mut r] = [0, pool.len()];
            while r - l > 1 {
                let m = l+(r-l>>1);
                if pool[m].len()+1 < len_max { l = m }
                else { r = m }
            };
            l
        };

        loop {
            if shift[dep] >= upper_bound { break None } // shifted out of the window

            let selected = pool[shift[dep]];
            assert!(selected.len() <= len_max);

            if dep >= format.len()-1 { // last iteration (base case)
                if selected.len() < len_min { break None } // would wrap all the way around 
                break Some(vec![selected])
            }

            match self.generate_backtracking((len_min as i32 - selected.len() as i32).max(0) as usize,
                                             len_max - selected.len(),
                                             dep+1, shift, dict, format) {
                Some(mut suf) => {
                    suf.push(selected);
                    shift[dep] += 1;
                    break Some(suf)
                }
                _ => {
                    shift[dep] += 1;
                    //for i in dep+1 .. shift.len() { shift[i] = 0; }
                }
            }
        }
    }

    /// Generate a requested phrases if possible
    ///
    /// returns None if the conditions could not be satisfied
    pub fn generic(&self,
                   words: usize,
                   count: usize,
                   len_min: Option<usize>,
                   len_max: Option<usize>,
                   start_char: Option<char>
                ) -> Option<Vec<Vec<&'static str>>> {

        let len_min = len_min.unwrap_or(0);
        let len_max = len_max.unwrap_or(usize::MAX);

        if len_max < len_min        { return None }
        if words > 4 || words == 0  { return None }

        // convert to references
        let words_intensifiers = self.words_intensifiers.iter().map(|x| x).collect::<Vec<&&'static str>>();
        let words_adjectives   = self.words_adjectives  .iter().map(|x| x).collect::<Vec<&&'static str>>();
        let words_nouns        = self.words_nouns       .iter().map(|x| x).collect::<Vec<&&'static str>>();

        // dictionary that we can recurse over
        let mut dict = [Vec::new(), words_intensifiers, words_adjectives, words_nouns];

        for list in &mut dict {
            // filter words we know we can't use
            if let Some(c) = start_char {
                list.retain(|s| s.chars().nth(0).expect("empty word found!") == c);
            }
            list.retain(|s| s.len() < len_max);         // filter out words that are already longer than len_max
            list.shuffle(&mut *self.rng.borrow_mut());  // shuffle all the available words 
            list.sort_by(|a, b| a.len().cmp(&b.len())); // sort by length (stable sort, so still shuffled) for easier length matching
        }

        for list in &dict {
            println!("{:?}", list)
        }

        let mut ret = vec![vec![""; words]; count];
        let mut shift = [0 as usize; 4];

        for i in 0..count {
            if let Some(mut vec) = self.generate_backtracking(len_min, len_max, 1, &mut shift, &dict, &Generator::create_format(words)) {
                vec.reverse();
                ret[i] = vec;
            } else {
                return None;
            }
        }
        Some(ret) 
    }

    /// Generate a witty phrase with either 1, 2, or 3 words
    ///
    /// returns None when no phrase could be generated (eg. if one of the wordlists is empty)
    pub fn with_words(&self, words: usize) -> Option<Vec<&'static str>> {
        let mut ret = vec![""; words];
        let mut n = 0;

        if words > 3 { ret[3] = self.words_nouns       .iter().choose(&mut *self.rng.borrow_mut())?; }

        if words > 2 { ret[n] = self.words_intensifiers.iter().choose(&mut *self.rng.borrow_mut())?; n += 1; }
        if words > 1 { ret[n] = self.words_adjectives  .iter().choose(&mut *self.rng.borrow_mut())?; n += 1; }
        if words > 0 { ret[n] = self.words_nouns       .iter().choose(&mut *self.rng.borrow_mut())?; }

        Some(ret)
    }
}

