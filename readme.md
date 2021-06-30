# witty-phrase-generator

Generates three-word phrases of the form `intensifier-adjective-noun`, just like GitHub default repo names.

Has minimal dependencies (just `rand` and `getopts`) and minimal bloat. 

## Usage

```
$ witty-phrase-generator        # -> staggeringly-wise-alchohol
$ witty-phrase-generator -2     # -> fantastic brush
$ witty-phrase-generator -n 4   # (outputs 4 lines of 3-word phrases)
```

Also supports alliteration, max length, and other features. Use `witty-phrase-generator --help` to learn more!

## Improvements
- always looking for more witty words to add to the wordlist.
- alliteration and length algorithms are pretty random right now, and thus pretty slow if the constraints are tight.
