# witty-phrase-generator

Generates three-word phrases of the form `intensifier-adjective-noun`, just like GitHub default repo names.

Has minimal dependencies (just `rand` and `getopts`) and minimal bloat. Uses backtracking and binary search when constraints are tight to avoid repeated computation and maximize speed.

## Usage

```sh
$ witty-phrase-generator        # -> staggeringly-wise-alchohol
$ witty-phrase-generator -2     # -> fantastic brush
$ witty-phrase-generator -n 4   # (outputs 4 lines of 3-word phrases)
```

```rust
use witty_phrase_generator::WPGen;

fn main() {
    let wp_gen = WPGen::new();  // contains its own thread_rng
    wp_gen.generic(3,           // words per phrase
                   30,          // phrases
                   Some(25),    // minimum length
                   Some(25),    // maximum length
                   Some('a'),   // alliterate with 'a'
                   ).expect("Could not satisfy constraints!");
}
```

Also supports alliteration, max length, and other features. Use `witty-phrase-generator --help` or check the docstrings to learn more!

## Improvements
- always looking for more witty words to add to the wordlist.
- can have duplicates (common if constraints are tight)
- allow generating with a given length but variable number of words
- implement ablaut vowel rules for noun-noun phrases to make them sound nicer
