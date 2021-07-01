use rand::prelude::{ ThreadRng, thread_rng, IteratorRandom };
use rand::Rng;
use rand::seq::SliceRandom;

use std::cell::RefCell;

pub struct WPGen {
    rng: RefCell<ThreadRng>,
    words_intensifiers: Vec<&'static str>,
    words_adjectives: Vec<&'static str>,
    words_nouns: Vec<&'static str>,
}

impl WPGen {
    pub fn new() -> WPGen {
        let words_intensifiers = include_str!("intensifiers.txt");
        let words_adjectives   = include_str!("adjectives.txt")  ;
        let words_nouns        = include_str!("nouns.txt")       ;

        let words_intensifiers = words_intensifiers.lines().collect::<Vec<&'static str>>();
        let words_adjectives   = words_adjectives  .lines().collect::<Vec<&'static str>>();
        let words_nouns        = words_nouns       .lines().collect::<Vec<&'static str>>();

        WPGen { 
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

        if words > 3 { ret[4] = 3 }
        if words > 2 { ret[n] = 1; n += 1; }
        if words > 1 { ret[n] = 2; n += 1; }
        if words > 0 { ret[n] = 3; }
         
        ret
    }

    fn generate_backtracking(&self,
                             len_min: usize,
                             len_max: usize,
                             dep: usize,
                             dict: &[Vec<&&'static str>; 4],
                             format: &Vec<usize>,
                            ) -> Option<Vec<&'static str>> {
        let pool = &dict[format[dep]];
        
        //let upper_bound = {
        //    let [mut l, mut r] = [0, pool.len()];
        //    while r - l > 1 {
        //        let m = l+(r-l>>1);
        //        if pool[m].len() <= len_max +1 { l = m }
        //        else { r = m }
        //    };
        //    l
        //};
        // TODO: is binary search even faster on such short wordlists?
        let mut upper_bound = 0;
        while upper_bound < pool.len() && pool[upper_bound].len() <= len_max { upper_bound += 1; }


        let pool = pool[0..upper_bound]
                .choose_multiple(&mut *self.rng.borrow_mut(), upper_bound)
                .collect::<Vec<&&&str>>();

        for selected in pool {
            assert!(selected.len() <= len_max);

            if dep >= format.len()-1 { // last iteration (base case)
                if selected.len() < len_min { continue } // would wrap all the way around 
                return Some(vec![selected])
            }

            if let Some(mut suf) = self.generate_backtracking(
                    (len_min as i32 - selected.len() as i32).max(0) as usize,
                    len_max - selected.len(),
                    dep+1, dict, format) {
                suf.push(selected);
                return Some(suf);
            }
        };
        None
    }

    /// Generate the requested phrases if possible
    ///
    /// Returns None if the conditions could not be satisfied
    ///
    /// All words (even across phrases) will start with start_char
    /// if it is provided. To allow different phrases to alliterate
    /// with different letters, use with_phrasewise_alliteration
    /// instead. 
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
            list.retain(|s| s.len() <= len_max/(words-1));  // filter out words that are already longer than len_max; TODO: too restrictive sometimes
            list.shuffle(&mut *self.rng.borrow_mut());      // shuffle all the available words 
            list.sort_by(|a, b| a.len().cmp(&b.len()));     // sort by length (stable sort, so still shuffled) for easier length matching
        }

        let mut ret = vec![vec![""; words]; count];

        for i in 0..count {
            if let Some(mut vec) = self.generate_backtracking(len_min, len_max, 1, &dict, &WPGen::create_format(words)) {
                vec.reverse();
                ret[i] = vec;
            } else {
                return None;
            }
        }
        Some(ret) 
    }

    /// Generate the requested phrases if possible
    ///
    /// Returns None if the conditions could not be satisfied
    ///
    /// Each phrase will alliterate internally, but different
    /// phrases may start with different letters. To specify
    /// what letter to start with, use generic() instead.
    pub fn with_phrasewise_alliteration(&self,
                   words: usize,
                   count: usize,
                   len_min: Option<usize>,
                   len_max: Option<usize>,
                ) -> Option<Vec<Vec<&'static str>>> {

        let mut ret = Vec::new();
        ret.reserve_exact(count);
        for _ in 0..count {
            ret.append(&mut loop {
                let char = (*self.rng.borrow_mut()).gen_range(b'a'..b'z'+1) as char;
                if let Some(p) = self.generic(words, 1, len_min, len_max, Some(char)) {
                    break p
                }
            });
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

